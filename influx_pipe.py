


import influxdb_client
from influxdb_client.client.write_api import SYNCHRONOUS
import time

org = "coolcats"

def get_client():
    bucket = "ZZZZ"
    
    token = 'YgxPAR8D9Zd9Yz7UG1cN84fZHYtJSVVI5EQYf-Xgn4EqenrJ-76Gz1LqcHD9tBEQyMRWqPt80aRj6a7a6LekGg=='

    # Store the URL of your InfluxDB instance
    url="http://localhost:8086"

    client = influxdb_client.InfluxDBClient(
        url=url,
        token=token,
        org=org
    )
    return client


def list_datasets():
    bucket_objects = get_client().buckets_api().find_buckets()

    #print(bucket_objects.__dict__)
    bucket_names = [bucket.name for bucket in bucket_objects.buckets]
    #print(bucket_names)
    return bucket_names


def list_channels(dataset):
    query = f"""
    import \"influxdata/influxdb/schema\"

    schema.measurements(bucket: \"{dataset}\")
    """
        
    query_api = get_client().query_api()
    tables = query_api.query(query=query)
    measurements = [row.values["_value"] for table in tables for row in table]
    return measurements

def get_data(dataset, channel):
    #|> range(start: -10m)\
    
    query = f'from(bucket:"{dataset}")\
    |> range(start: -1d)\
    |> filter(fn:(r) => r._measurement == "{channel}")\
    |> sort(columns: ["_time"])\
    |> limit(n: 1000000)'

    data_records = get_client().query_api().query(query)
    result = []
    useful_data = data_records.to_values(columns=['_time', '_value'])
    sorted_data = sorted(useful_data, key=lambda tup: tup[0])
    for tm_datetime, value in sorted_data:
        unixtime = tm_datetime.timestamp()#time.mktime(tm_datetime.timetuple())
        #print(unixtime, value)
        result.append([unixtime, value])
    return result

'''
def get_data(dataset, channel):
    #|> range(start: -10m)\
    
    query = f'SELECT * from {channel}'

    data_records = get_client().query_api().query(query)
    result = []
    for tm_datetime, value in data_records.to_values(columns=['_time', '_value']):
        unixtime = tm_datetime.timestamp()#time.mktime(tm_datetime.timetuple())
        #print(unixtime, value)
        result.append([unixtime, value])
    return result
'''
#dataset = 'the_main_bucket'
#query = f'from(bucket:"{dataset}")\
#|> range(start: -10m)\
#|> filter(fn:(r) => r._measurement == "task_scheduler_current_execution")'
#
#loginRecords = get_client().query_api().query(query)
#
#
#
#print(loginRecords.to_json())
#for tm_datetime, value in loginRecords.to_values(columns=['_time', '_value']):
#    unixtime = tm_datetime.timestamp()#time.mktime(tm_datetime.timetuple())
#    print(unixtime, value)

#print(loginRecords.Columns)
#print(loginRecords)

#get_data('the_main_bucket', 'task_scheduler_current_execution')