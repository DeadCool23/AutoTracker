import generators.sts_gen as stss
import generators.cars_gen as cars
import generators.snaps_gen as snaps
import generators.users_gen as users
import generators.cameras_gen as cams
import generators.owners_gen as owners
import generators.pts_gen as ptss

import pandas as pd

from gened_data_files import *

CAMS_CNT = 50
CARS_CNT = 2000
SNAPS_CNT = 200
USERS_CNT = 200
OWNERS_CNT = 1000

if "__main__" == __name__:
    # Получение уже сгенерированных данных
    # cam_df = pd.read_csv(CAMERAS_DATA_FILE)
    # sts_df = pd.read_csv(STSS_DATA_FILE)
    # owner_df = pd.read_csv(OWNERS_DATA_FILE)
    # car_df = pd.read_csv(CARS_DATA_FILE)
    # user_df = pd.read_csv(USERS_DATA_FILE)

    # print("===ГЕНЕРАЦИЯ ВЛАДЕЛЬЦЕВ===")
    # owner_df = owners.gen_rand_car_owners(OWNERS_CNT, user_df)
    # save_to_csv(OWNERS_DATA_FILE, owner_df)
    # print(f"Владельцы успешно сгенерированы и сохранены в {OWNERS_DATA_FILE}\n\n")

    # print("===ГЕНЕРАЦИЯ АВТОМОБИЛЕЙ===")
    # car_df = cars.gen_rand_cars(CARS_CNT, owner_df)
    # save_to_csv(CARS_DATA_FILE, car_df)
    # print(f"Автомобили успешно сгенерированы и сохранены в {CARS_DATA_FILE}\n\n")

    # print("===ГЕНЕРАЦИЯ ПОЛЬЗОВАТЕЛЕЙ===")
    # user_df = users.gen_rand_users(USERS_CNT)
    # save_to_csv(USERS_DATA_FILE, user_df)
    # print(f"Пользователи успешно сгенерированы и сохранены в {USERS_DATA_FILE}\n\n")

    # print("===ГЕНЕРАЦИЯ СТС===")
    # sts_df = stss.gen_rand_sts(car_df, owner_df)
    # save_to_csv(STSS_DATA_FILE, sts_df)
    # print(f"СТС успешно сгенерированы и сохранены в {STSS_DATA_FILE}\n\n")

    # print("===ГЕНЕРАЦИЯ ПТС===")
    # sts_df = ptss.gen_rand_pts(sts_df)
    # save_to_csv(PTSS_DATA_FILE, sts_df)
    # print(f"ПТС успешно сгенерированы и сохранены в {PTSS_DATA_FILE}\n\n")

    # print("===ГЕНЕРАЦИЯ КАМЕР===")
    # cam_df = cams.gen_rand_cameras(CAMS_CNT)
    # save_to_csv(CAMERAS_DATA_FILE, cam_df)
    # print(f"Камеры успешно сгенерированы и сохранены в {CAMERAS_DATA_FILE}\n\n")

    # print("===ГЕНЕРАЦИЯ СНИМКОВ АВТО===")
    # snap_df = snaps.gen_rand_car_snapshots(SNAPS_CNT, cam_df, sts_df)
    # save_to_csv(SNAPS_DATA_FILE, snap_df)
    # print(f"Снимки авто успешно сгенерированы и сохранены в {SNAPS_DATA_FILE}\n\n")
    pass