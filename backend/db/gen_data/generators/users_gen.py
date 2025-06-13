import random
import pandas as pd
from faker import Faker
import string
from collections import defaultdict

ROLES = ('user', 'operator', 'admin')
ROLES_WEIGHT = [80, 15, 5]  # 80% user, 15% operator, 5% admin

def gen_rand_users(n: int) -> pd.DataFrame:
    fake = Faker('ru_RU')
    used_passports = defaultdict(bool)

    def generate_random_email():
        username_options = [
            ''.join(random.choices(string.ascii_lowercase, k=random.randint(5, 10))),
            ''.join(random.choices(string.ascii_lowercase + string.digits, k=8))
        ]
        
        username = random.choice(username_options)
        domain = random.choice([
            'gmail.com', 'yahoo.com', 'outlook.com', 'hotmail.com',
            'protonmail.com', 'icloud.com', 'mail.com', 'zoho.com'
        ])
        
        return f"{username}@{domain}"
    
    def generate_password():
        chars = string.ascii_letters + string.digits + '!@#$%^&*'
        return ''.join(random.choices(chars, k=random.randint(8, 12)))
    
    def generate_unique_passport() -> tuple[int, int]:
        while True:
            serial = random.randint(1000, 9999)
            number = random.randint(100000, 999999)
            passport_key = f"{serial}_{number}"
            
            if not used_passports[passport_key]:
                used_passports[passport_key] = True
                return serial, number
    
    data = {
        'login': [],
        'password': [],
        'role': [],
        'name': [],
        'surname': [],
        'lastname': [],
        'is_verified': [],
        'passport_serial': [],
        'passport_num': []
    }
    
    for _ in range(n):
        name = fake.first_name()
        surname = fake.last_name()
        lastname = fake.middle_name() if random.random() > 0.3 else None
        
        login = generate_random_email()
        password = generate_password()
        role = random.choices(ROLES, weights=ROLES_WEIGHT)[0]
        
        if role in ('admin', 'operator'):
            is_verified = True
            passport_serial, passport_num = generate_unique_passport()
        else:
            is_verified = random.random() > 0.2
            if is_verified:
                passport_serial, passport_num = generate_unique_passport()
            else:
                passport_serial, passport_num = None, None
        
        data['login'].append(login)
        data['password'].append(password)
        data['role'].append(role)
        data['name'].append(name)
        data['surname'].append(surname)
        data['lastname'].append(lastname)
        data['is_verified'].append(is_verified)
        data['passport_serial'].append(passport_serial)
        data['passport_num'].append(passport_num)
    
    df = pd.DataFrame(data)
    df['passport_serial'] = df['passport_serial'].astype('Int64')
    df['passport_num'] = df['passport_num'].astype('Int64')

    return df