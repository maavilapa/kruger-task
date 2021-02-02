# kruger-task
    "
    Bienvenido!
    En este servicio puedes hacer la migración de tus datos desde un archivo CSV a una base de datos en PostgreSQL.
    Primero se debe ejecutar el archivo principal para inciar el servicio:
    
                        cargo run --bin main
                        
    Al servicio se puede acceder en  http://localhost:3000 y se deben seguir los pasos.
    
    En http://localhost:3000/upload puedes ver si los datos ya fueron cargados correctamente.

    Para cargarlos guarda el archivo en la carpeta upload y luego ejecuta dentro del proyecto el comando:

                        curl --data-raw @upload/data.csv http://localhost:3000/upload

    En la terminal se mostrarán los valores almacenados para cada persona y se puede verificar haciendo una búsqueda en
    base de datos
                        cargo run --bin query
    "
