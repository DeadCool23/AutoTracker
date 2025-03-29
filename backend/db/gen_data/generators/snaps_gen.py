import random
import pandas as pd
from datetime import datetime, timedelta

def gen_rand_car_snapshots(n: int, cameras_df: pd.DataFrame, sts_df: pd.DataFrame) -> pd.DataFrame:
    if len(cameras_df) == 0:
        raise ValueError("Нет данных о камерах")
    if len(sts_df) == 0:
        raise ValueError("Нет данных об автомобилях")
    
    cameras_df = cameras_df.copy()
    cameras_df['install_date'] = pd.to_datetime(cameras_df['install_date'])
    
    camera_data = cameras_df[['id', 'install_date']].to_dict('records')
    gos_nums = sts_df['gos_num'].dropna().unique().tolist()
    
    if not gos_nums:
        raise ValueError("Нет автомобилей с гос. номерами")
    
    data = {
        'camera_id': [],
        'gos_num': [],
        'road_line': [],
        'speed': [],
        'snap_datetime': []
    }
    
    end_date = datetime.now()
    
    for _ in range(n):
        camera = random.choice(camera_data)
        camera_id = camera['id']
        install_date = camera['install_date']
        
        min_date = max(install_date, end_date - timedelta(days=365))
        max_date = end_date
        
        random_seconds = random.randint(0, int((max_date - min_date).total_seconds()))
        date = min_date + timedelta(seconds=random_seconds)
        
        gos_num = random.choice(gos_nums)
        
        road_line = random.randint(1, 5)
        speed = random.randint(20, 180)
        
        data['camera_id'].append(camera_id)
        data['gos_num'].append(gos_num)
        data['road_line'].append(road_line)
        data['speed'].append(speed)
        data['date'].append(date.strftime('%Y-%m-%d %H:%M:%S'))
    
    return pd.DataFrame(data)