import string
import random
import pandas as pd
from faker import Faker
from datetime import datetime, timedelta

def gen_rand_sts(cars_df, owners_df):
    fake = Faker('ru_RU')
    
    if len(cars_df) == 0:
        raise ValueError("Нет данных об автомобилях")
    if len(owners_df) == 0:
        raise ValueError("Нет данных о владельцах")
    
    used_sts_numbers = set()
    used_car_ids = set()
    
    def generate_sts_number():
        while True:
            serial = random.randint(1000, 9999)
            num = random.randint(100000, 999999)
            if (serial, num) not in used_sts_numbers:
                used_sts_numbers.add((serial, num))
                return serial, num
    
    engine_types = ['petrol', 'diesel', 'electric', 'hybrid']
    car_classes = ['A', 'B', 'C', 'D', 'E', 'F', 'S', 'M', 'J']
    
    def generate_gos_num():
        letters = 'АВЕКМНОРСТУХ'
        return (f"{random.choice(letters)}"
                f"{random.randint(0, 9)}{random.randint(0, 9)}{random.randint(0, 9)}"
                f"{random.choice(letters)}{random.choice(letters)}"
                f"{random.randint(0, 9)}{random.randint(0, 9)}")
    
    data = {
        'car_id': [],
        'owner_id': [],
        'vin': [],
        'gos_num': [],
        'mark': [],
        'model': [],
        'horse_power': [],
        'car_weight': [],
        'sts_serial': [],
        'sts_num': [],
        'engine_type': [],
        'car_class': [],
        'release_date': [],
        'reg_date': []
    }
    
    car_records = cars_df.to_dict('records')
    random.shuffle(car_records)
    
    for car in car_records:
        car_id = car.get('id')
        if car_id in used_car_ids:
            continue
            
        used_car_ids.add(car_id)
        
        owner = random.choice(owners_df.to_dict('records'))
        
        serial, num = generate_sts_number()
        gos_num = generate_gos_num()
        
        release_date = fake.date_between(
            start_date=datetime.now() - timedelta(days=20*365),
            end_date='today'
        )
        reg_date = fake.date_between(
            start_date=release_date,
            end_date='today'
        )
        
        horse_power = random.randint(60, 800)
        weight = random.randint(800, 3500)
        engine_type = random.choice(engine_types)
        car_class = random.choice(car_classes)
        
        data['car_id'].append(car_id)
        data['owner_id'].append(owner.get('id'))
        data['vin'].append(car['vin'])
        data['gos_num'].append(gos_num)
        data['mark'].append(car['mark'])
        data['model'].append(car['model'])
        data['horse_power'].append(horse_power)
        data['car_weight'].append(weight)
        data['engine_type'].append(engine_type)
        data['car_class'].append(car_class)
        data['release_date'].append(release_date.strftime('%Y-%m-%d'))
        data['reg_date'].append(reg_date.strftime('%Y-%m-%d'))
        data['sts_serial'].append(serial)
        data['sts_num'].append(num)
    
    df = pd.DataFrame(data)
    
    int_cols = ['serial', 'num', 'horse_power', 'weight', 'car_id', 'owner_id']
    for col in int_cols:
        if col in df.columns:
            try:
                df[col] = pd.to_numeric(df[col]).astype('Int64')
            except (ValueError, TypeError):
                continue
    
    return df