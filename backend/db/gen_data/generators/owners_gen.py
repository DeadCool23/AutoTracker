import random
import pandas as pd
from faker import Faker
from datetime import datetime, timedelta

def gen_rand_car_owners(n: int, cars_df: pd.DataFrame, users_df: pd.DataFrame | None = None, user_match: float = 0.6) -> pd.DataFrame:
    fake = Faker('ru_RU')
    
    if len(cars_df) == 0:
        raise ValueError("Нет данных об автомобилях")
    
    car_indices = list(range(len(cars_df)))
    
    user_data_available = users_df is not None and len(users_df) > 0
    
    if user_data_available:
        potential_users = users_df[
            (users_df['passport_serial'].notnull()) & 
            (users_df['passport_num'].notnull())
        ].copy()
        
        passport_to_user = {
            (row['passport_serial'], row['passport_num']): row
            for _, row in potential_users.iterrows()
        }
    else:
        passport_to_user = {}
    
    data = {
        'car_id': [],
        'name': [],
        'surname': [],
        'lastname': [],
        'age': [],
        'passport_serial': [],
        'passport_num': [],
        'drive_exp': [],
        'drive_license_serial': [],
        'drive_license_num': [],
    }
    
    used_passports = set()
    used_licenses = set()
    
    def generate_passport():
        while True:
            serial = random.randint(1000, 9999)
            number = random.randint(100000, 999999)
            passport = (serial, number)
            if passport not in used_passports:
                used_passports.add(passport)
                return passport
    
    def generate_license():
        while True:
            serial = random.randint(1000, 9999)
            number = random.randint(100000, 999999)
            license = (serial, number)
            if license not in used_licenses:
                used_licenses.add(license)
                return license
    
    for _ in range(n):
        car_index = random.choice(car_indices)
        
        is_user = user_data_available and random.random() < user_match
        
        if is_user and passport_to_user:
            passport, user = random.choice(list(passport_to_user.items()))
            
            name = user['name']
            surname = user['surname']
            lastname = user.get('lastname', None)
            age = random.randint(18, 90)
            passport_serial, passport_num = passport
        else:
            name = fake.first_name()
            surname = fake.last_name()
            lastname = fake.middle_name() if random.random() > 0.3 else None
            age = random.randint(18, 90)
            passport_serial, passport_num = generate_passport()
        
        drive_exp = random.randint(0, age - 18)
        
        license_serial, license_num = generate_license()
        
        data['car_index'].append(car_index)
        data['name'].append(name)
        data['surname'].append(surname)
        data['lastname'].append(lastname)
        data['age'].append(age)
        data['passport_serial'].append(passport_serial)
        data['passport_num'].append(passport_num)
        data['drive_exp'].append(drive_exp)
        data['drive_license_serial'].append(license_serial)
        data['drive_license_num'].append(license_num)
    
    df = pd.DataFrame(data)
    df['passport_serial'] = df['passport_serial'].astype('Int64')
    df['passport_num'] = df['passport_num'].astype('Int64')

    return df