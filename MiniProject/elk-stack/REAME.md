# ELK Stack example

### Configuration

1. Install

```bash

wget -p0 - https://artifacts.elastic.co/GPG-KEY-elasticsearch | sudo gpg --dearmor - o /usr/share/keyring/elasticsearch-keyring.gpg

sudo apt-get install apt-transport-https

echo "deb [signed-by=/usr/share/keyrings/elasticsearch-keyring.gpg] https://artifacts.elastic.co/packages/8.x/apt stable main" | sudo tee /etc/apt/sources.list.d/elastic-8.x.list

sudo apt-get update && sudo apt-get install elasticsearch

sudo apt-get install kibana
sudo apt-get install logsatsh
sudo apt-get install filebeat

sudo /bin/systemctl daemon-reload
sudo /bin/systemctl enable elasticsearch.service
sudo /bin/systemctl enable filebeat
sudo /bin/systemctl enable logstash.service
sudo /bin/systemctl enable kibana.service

### For check 
ps aux | grep elastic
ps aux | grep filebeat
ps aux | grep kibana
ps aux | grep logstash

sudo apt-get install nginx-full

```

2. Filebeat

```bash
vim /etc/filebeat/filebeat.yml # Follow filebeat.yml
sudo /bin/systemctl stop filebeat
sudo /bin/systemctl start filebeat
```

3. Logstash

```bash
vim /etc/logstash/conf.d/nginx.conf

```

4. Kibana

```bash
vim kibana.yml

# Change serverhost: localhost => 0.0.0.0
```

5. Elasticsearch

- Go to localhost:5601

- For auto config

```
/usr/share/elasticsearch/bin/elasticsearch-create-enrollment-token --scope kibana
```

- Get code

```
/usr/share/kibana/bin/kibana-verification-code 
```
- Password

```
/usr/share/elasticsearch/bin/elasticsearch-reset-password -u elastic
```












