import random
import string
import pandas as pd
from datetime import datetime, timedelta

COLORS = ['Black', 'White', 'Silver', 'Gray', 'Red', 'Blue', 'Green', 'Yellow']
CAR_MARKS = ['Toyota', 'Honda', 'Ford', 'BMW', 'Mercedes', 'Audi', 'Volkswagen', 'Hyundai', 'Kia', 'Lexus']
CAR_MODELS = {
        'Toyota': ['Camry', 'Corolla', 'RAV4', 'Prius', 'Land Cruiser'],
        'Honda': ['Accord', 'Civic', 'CR-V', 'Pilot', 'Fit'],
        'Ford': ['Focus', 'Fiesta', 'Mustang', 'Explorer', 'F-150'],
        'BMW': ['3 Series', '5 Series', 'X5', 'X3', '7 Series'],
        'Mercedes': ['G-Class', 'C-Class', 'E-Class', 'S-Class', 'GLC', 'GLE'],
        'Audi': ['A4', 'A6', 'Q5', 'Q7', 'A3'],
        'Volkswagen': ['Golf', 'Passat', 'Tiguan', 'Polo', 'Touareg'],
        'Hyundai': ['Solaris', 'Tucson', 'Santa Fe', 'Creta', 'Elantra'],
        'Kia': ['Rio', 'Sportage', 'Sorento', 'Optima', 'Cerato'],
        'Lexus': ['RX', 'NX', 'ES', 'LS', 'GX']
    }

def gen_rand_cars(n: int, owner_df: pd.DataFrame) -> pd.DataFrame:
    def generate_vin():
        chars = string.ascii_uppercase + string.digits
        return ''.join(random.choices(chars, k=17))
    
    data = {
        'owner_id': [],
        'mark': [],
        'model': [],
        'vin': [],
        'mileage': [],
        'color': []
    }
    
    for _ in range(n):
        owner_id = random.choice(owner_df['id'])
        mark = random.choice(CAR_MARKS)
        model = random.choice(CAR_MODELS[mark])
        vin = generate_vin()
        mileage = random.randint(0, 300000)
        color = random.choice(COLORS)
        
        data['owner_id'].append(owner_id)
        data['mark'].append(mark)
        data['model'].append(model)
        data['vin'].append(vin)
        data['mileage'].append(mileage)
        data['color'].append(color)

    df = pd.DataFrame(data)
    return df