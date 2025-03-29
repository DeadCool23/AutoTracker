import os
import pandas as pd
from sqlalchemy import create_engine
from dotenv import load_dotenv
from tqdm import tqdm

load_dotenv()

def load_data_to_db(csv_file: str, table_name: str):
    DATABASE_URL = os.getenv('POSTGRES_URL')
    if not DATABASE_URL:
        raise ValueError("Не задан DATABASE_URL в .env файле")
    
    engine = create_engine(DATABASE_URL)
    
    df = pd.read_csv(csv_file)
    
    with tqdm(total=len(df), desc=f"Загрузка {table_name}") as pbar:
        for i in range(0, len(df), 1000):
            batch = df.iloc[i:i+1000]
            batch.to_sql(
                table_name,
                engine,
                if_exists='append',
                index=False,
                method='multi'
            )
            pbar.update(len(batch))
    
    print(f"Данные успешно загружены в таблицу {table_name}")

if __name__ == "__main__":
    DATA_DIR = "./data"

    data_to_load = {
        f'{DATA_DIR}/users.csv': 'appuser',
        f'{DATA_DIR}/cars.csv': 'car',
        f'{DATA_DIR}/owners.csv': 'carowner',
        f'{DATA_DIR}/stss.csv': 'sts',
        f'{DATA_DIR}/snaps.csv': 'carsnapshot',
        f'{DATA_DIR}/cameras.csv': 'camera',
        f'{DATA_DIR}/ptss.csv': 'pts',
        f'{DATA_DIR}/track_info.csv': 'trackinfo',
        f'{DATA_DIR}/ownership_history.csv': 'ownershiphistory',
        f'{DATA_DIR}/ownership_history_owner.csv': 'ownerhistoryowner',
    }
    
    for csv_file, table_name in data_to_load.items():
        if os.path.exists(csv_file):
            try:
                load_data_to_db(csv_file, table_name)
            except Exception as e:
                print(f"Ошибка при загрузке {csv_file}: {str(e)}")
        else:
            print(f"Файл {csv_file} не найден, пропускаем")