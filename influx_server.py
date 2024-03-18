
from flask import Flask, jsonify
#from flask.ext.cors import CORS,
from flask_cors import CORS

import influx_pipe

app = Flask(__name__)
#cors = CORS(app, resources={r"/data": {"origins": "*"}})
#app.config['CORS_HEADERS'] = 'Content-Type'
CORS(app)

@app.route('/list_datasets/<filter>')
def list_datasets(filter):
    return jsonify({"datasets":influx_pipe.list_datasets()})

@app.route('/list_channels/<dataset>/<filter>')
def list_channels(dataset, filter):
    return jsonify({"channels":influx_pipe.list_channels(dataset)})

@app.route('/data/<dataset>/<channel>')
def data(dataset, channel):
    #dataset = 'the_main_bucket'
    data = influx_pipe.get_data(dataset, channel)
    return jsonify({"data":data}) 


    '''
    result = []
    full_range = 120
    for ix in range(full_range):
        x = float(ix)
        y = (x / full_range) * 3.14;
        result.append([x, y])
    return jsonify({'data':result})
    '''

if __name__ == '__main__':
      app.run(host='0.0.0.0', port=8081)