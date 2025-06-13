import random
import pandas as pd
from datetime import datetime, timedelta

CAMS_DATA_FILE_PATH = './gen_data/generators/help_data/cams_data.xlsx'

def gen_rand_cameras(n: int) -> pd.DataFrame:
    def extract_random_cams_coords(n):
        df = pd.read_excel(CAMS_DATA_FILE_PATH)
        
        random_sample = df.sample(n)
        
        latitudes = random_sample['Широта'].tolist()
        longitudes = random_sample['Долгота'].tolist()
        
        return (latitudes, longitudes)

    lat, lon = extract_random_cams_coords(n)
    data = {
        'longitude': lon,
        'latitude': lat,
        'install_date': [],
        'is_radar': []
    }

    for _ in range(n):
        start_date = datetime(2018, 1, 1)
        end_date = datetime(2022, 12, 31)
        random_date = start_date + timedelta(days=random.randint(0, (end_date - start_date).days))
        
        is_radar = random.choice([True, False])
        
        data['install_date'].append(random_date.strftime('%Y-%m-%d'))
        data['is_radar'].append(is_radar)

    df = pd.DataFrame(data)
    return df
