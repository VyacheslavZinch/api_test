                                                Example of a simple Web API.


        The project uses :        

        - the Rust programming language
        - PostgreSQL database 
        - ORM Diesel 
        - Rocket Web-framework 
        - Mongo DB
        

        Routes:
           >>  GET /seats
           >>  POST /seats         application/json
           >>  GET /files
           >>  GET /aircrafts
           >>  GET /boardpasses
           >>  POST /boardpasses   application/json
           >>  GET /aircrafts/<id>
           >>  GET /boardpasses/<id>
           >>  DELETE /boardpasses/<id>
           >>  GET /file/<sample.file>

        Rocket limits:
            form = "512 kB"
            json = "10 MiB"
            msgpack = "20 MiB"
            "file/jpg" = "50 MiB"


        Authorization by token. 

            

    The database was taken from this resource (including description) - https://edu.postgrespro.ru/bookings.pdf



