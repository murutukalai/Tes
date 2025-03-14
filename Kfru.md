Yes, you can download Kafka as a tar.gz file, extract it, and execute it without needing to install anything system-wide. Here’s how:  

---

## **1. Download Kafka Executable File**
Run the following command to download Kafka (replace the version if needed):  

```sh
wget https://downloads.apache.org/kafka/3.6.0/kafka_2.13-3.6.0.tgz
```

---

## **2. Extract Kafka**
```sh
tar -xzf kafka_2.13-3.6.0.tgz
cd kafka_2.13-3.6.0
```

---

## **3. Start Kafka Services**
Kafka requires **Zookeeper**, so start it first:

```sh
bin/zookeeper-server-start.sh config/zookeeper.properties
```

Then, open another terminal and start the Kafka broker:

```sh
bin/kafka-server-start.sh config/server.properties
```

---

## **4. Verify Kafka is Running**
To check if Kafka is running properly, list topics:

```sh
bin/kafka-topics.sh --list --bootstrap-server localhost:9092
```

---

## **5. Run Kafka Without Extracting (Optional)**
If you don’t want to extract Kafka and want to execute it directly, you can do:

```sh
mkdir kafka
cd kafka
wget https://downloads.apache.org/kafka/3.6.0/kafka_2.13-3.6.0.tgz
tar -xzf kafka_2.13-3.6.0.tgz --strip-components=1
bin/zookeeper-server-start.sh config/zookeeper.properties &
bin/kafka-server-start.sh config/server.properties &
```

This downloads, extracts, and runs Kafka in a single step.

---

Let me know if you need a script to automate this process!
