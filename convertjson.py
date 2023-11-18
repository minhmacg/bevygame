import json
import datetime
import pandas as pd
import os

def get_json_files(directory):
    return [pos_json for pos_json in os.listdir(directory) 
            if pos_json.endswith('.json')]

json_files = get_json_files('jsonbeat')
print(json_files)
def convertjson(files):
    for fs in files:
        with open(f"jsonbeat/{fs}", 'r', encoding="raw-unicode-escape") as fp:
            data = json.load(fp, strict=False)
        convert_hex_to_normal_letters(data)
        with open(f"jsonbeat/output_{fs}", 'w', encoding="raw-unicode-escape") as fp:
            json.dump(data, fp, ensure_ascii=False, indent=4)

def decode_hex(s):
    return bytes.fromhex(s).decode('raw-unicode-escape')

def convert_hex_to_normal_letters(data):
    if isinstance(data, dict):    #check xem co phai dict khong
        for key, value in data.items():
            if isinstance(value, str) and value.startswith("\\u"):
                data[key] = decode_hex(value)
            else:
                convert_hex_to_normal_letters(value)
    elif isinstance(data, list):
        for item in data:
            convert_hex_to_normal_letters(item)


# def converttime(data):
#     for message in data['messages']:
#         timestamp_ms = message["timestamp_ms"]
#         timestamp_s = timestamp_ms / 1000
#         dt_object = datetime.datetime.fromtimestamp(timestamp_s)
#         formatted_timestamp_date = dt_object.strftime("%d/%m/%Y")
#         formatted_timestamp_hour = dt_object.strftime("%H:%M:%S")
#         message["timestamp_ms"] = formatted_timestamp_date
#         message["time_hour"] = formatted_timestamp_hour
def main():
    print('Converting time...')
    print('Converting hex to unicode')
    convertjson(json_files)
if __name__ == "__main__":
    main()
