import random
import pandas as pd
from faker import Faker
from datetime import datetime, timedelta

import random
import pandas as pd
from faker import Faker
from datetime import datetime, timedelta

def gen_rand_car_owners(n: int, users_df: pd.DataFrame | None = None, user_match: float = 0.6) -> pd.DataFrame:
    fake = Faker('ru_RU')
    
    user_data_available = users_df is not None and len(users_df) > 0
    
    if user_data_available:
        potential_users = users_df[
            (users_df['passport_serial'].notnull()) & 
            (users_df['passport_num'].notnull())
        ].copy()
        user_records = potential_users.to_dict('records')
    else:
        user_records = []
    
    data = {
        'name': [],
        'surname': [],
        'lastname': [],
        'age': [],
        'drive_exp': [],
        'passport_serial': [],
        'passport_num': [],
        'drive_license_serial': [],
        'drive_license_num': []
    }
    
    used_passports = set()
    used_licenses = set()
    
    def generate_unique_passport():
        while True:
            serial = random.randint(1000, 9999)
            number = random.randint(100000, 999999)
            passport = (serial, number)
            if passport not in used_passports:
                used_passports.add(passport)
                return passport
    
    def generate_unique_license():
        while True:
            serial = random.randint(1000, 9999)
            number = random.randint(100000, 999999)
            license = (serial, number)
            if license not in used_licenses:
                used_licenses.add(license)
                return license
    
    for i in range(n):
        is_user = user_data_available and user_records and random.random() < user_match
        
        if is_user:
            user = random.choice(user_records)
            user_records.remove(user)
            
            name = user['name']
            surname = user['surname']
            lastname = user.get('lastname', None)
            age = random.randint(18, 90)
            passport_serial = user['passport_serial']
            passport_num = user['passport_num']
            
            used_passports.add((passport_serial, passport_num))
        else:
            name = fake.first_name()
            surname = fake.last_name()
            lastname = fake.middle_name() if random.random() > 0.3 else None
            age = random.randint(18, 90)
            passport_serial, passport_num = generate_unique_passport()
        
        drive_exp = random.randint(0, max(0, age - 18))
        license_serial, license_num = generate_unique_license()
        
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