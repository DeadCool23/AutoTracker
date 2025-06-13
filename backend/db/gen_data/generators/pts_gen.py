import random
import pandas as pd
from datetime import datetime, timedelta

COUNTRIES = [
    "Германия", "Грузия", "Армения", "Америка", "Великобритания", "Франция", "Япония", "Китай"
]

def gen_rand_pts(sts_df):
    if len(sts_df) == 0:
        raise ValueError("Нет данных об СТС")
    
    used_pts_numbers = set()
    used_sts_ids = set()
    
    def generate_pts_number():
        while True:
            serial = random.randint(1000, 9999)
            num = random.randint(100000, 999999)
            if (serial, num) not in used_pts_numbers:
                used_pts_numbers.add((serial, num))
                return serial, num
    
    data = {
        'sts_id': [],
        'pts_serial': [],
        'pts_number': [],
        'import_country': [],
    }
    
    sts_records = sts_df.to_dict('records')
    random.shuffle(sts_records)
    
    for sts in sts_records:
        sts_id = sts.get('id')
        if sts_id in used_sts_ids:
            continue
            
        used_sts_ids.add(sts_id)
        
        serial, num = generate_pts_number()
        
        import_country = random.choice(COUNTRIES)
        
        data['sts_id'].append(sts_id)
        data['pts_serial'].append(serial)
        data['pts_number'].append(num)
        data['import_country'].append(import_country)
    
    df = pd.DataFrame(data)
    
    int_cols = ['sts_id', 'pts_serial', 'pts_number']
    for col in int_cols:
        if col in df.columns:
            try:
                df[col] = pd.to_numeric(df[col]).astype('Int64')
            except (ValueError, TypeError):
                continue
    
    return df