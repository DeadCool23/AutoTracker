import pandas as pd

GEN_DATA_DIR = "./data"

CAMERAS_DATA_FILE = f"{GEN_DATA_DIR}/cameras.csv"
SNAPS_DATA_FILE = f"{GEN_DATA_DIR}/snaps.csv"
USERS_DATA_FILE = f"{GEN_DATA_DIR}/users.csv"
CARS_DATA_FILE = f"{GEN_DATA_DIR}/cars.csv"
TRACK_INFO_DATA_FILE = f"{GEN_DATA_DIR}/track_info.csv"
OWNERS_DATA_FILE = f"{GEN_DATA_DIR}/owners.csv"
PTSS_DATA_FILE = f"{GEN_DATA_DIR}/ptss.csv"
STSS_DATA_FILE = f"{GEN_DATA_DIR}/stss.csv"
OWNERSHIP_HISTORY_DATA_FILE = f"{GEN_DATA_DIR}/owner_history.csv"
OWNERSHIP_HISTORY_OWNER_DATA_FILE = f"{GEN_DATA_DIR}/ownership_history_owner.csv"

def save_to_csv(file_path: str, df: pd.DataFrame):
    df.insert(0, 'id', range(1, len(df) + 1))
    df.to_csv(file_path, index=False)