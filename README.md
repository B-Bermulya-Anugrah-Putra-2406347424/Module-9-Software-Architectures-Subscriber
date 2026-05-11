## Event-Driven Architecture: Subscriber and Message Broker

### a. What is amqp?
AMQP (*Advanced Message Queuing Protocol*) adalah sebuah protokol standar terbuka (*open standard*) di *application layer* yang digunakan oleh *message broker* (seperti RabbitMQ). Protokol ini dirancang khusus untuk komunikasi *message-oriented middleware*, memastikan pesan dikirimkan secara aman, andal, dan *interoperable* antar sistem atau aplikasi yang berbeda dalam arsitektur terdistribusi.

### b. What does it mean? `guest:guest@localhost:5672`, what is the first `guest`, and what is the second `guest`, and what is `localhost:5672` is for?
String `guest:guest@localhost:5672` adalah format URL untuk mendefinisikan kredensial dan alamat koneksi ke *message broker* RabbitMQ. Rinciannya sebagai berikut:
* **`guest` (pertama):** Merupakan *username* bawaan (*default*) untuk melakukan autentikasi ke RabbitMQ.
* **`guest` (kedua):** Merupakan *password* *default* untuk *username* tersebut.
* **`localhost:5672`:** Menunjukkan bahwa server RabbitMQ sedang berjalan di *machine* lokal (`localhost`) dan mendengarkan (*listening*) koneksi masuk pada *port* `5672`, yang mana merupakan *port* standar untuk protokol AMQP.