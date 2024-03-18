
from flask import Flask, jsonify
#from flask.ext.cors import CORS,
from flask_cors import CORS

app = Flask(__name__)
#cors = CORS(app, resources={r"/data": {"origins": "*"}})
#app.config['CORS_HEADERS'] = 'Content-Type'
CORS(app)

@app.route('/list_datasets/<filter>')
def list_datasets(filter):
    return jsonify({"datasets":['dataset1', 'dataset2', 'dataset3']})

@app.route('/list_channels/dataset/<filter>')
def list_channels(dataset, filter):
    if dataset in ['dataset1', 'dataset2', 'dataset3']:
        return jsonify({"channels":['awesome channel', 'badass channel']})
    else:
        return jsonify({"channels":[]})

@app.route('/data/<channel_id>')
def data(channel_id):
    result = []
    full_range = 120
    for ix in range(full_range):
        x = float(ix)
        y = (x / full_range) * 3.14;
        result.append([x, y])
    return jsonify({'data':result})


if __name__ == '__main__':
      app.run(host='0.0.0.0', port=8081)