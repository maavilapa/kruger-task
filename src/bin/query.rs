#![feature(proc_macro_hygiene, decl_macro)]



use postgres::{Client, NoTls, Error};


struct Persona {
    _id: i64,
    _nombre: String,
    _genero: String,
    _estado_civil: String,
    _fecha_nacimiento: String,
    _telefono: String,
    _direccion: String,
    _email: String,
    _valido: i32,
    _observaciones: String

}

pub fn main() -> Result<(), Error> {
   let mut client = Client::connect("postgresql://postgres:avila2017@localhost:5432/postgres", NoTls)?;



    // Retrieve the data from the table.
    for row in client.query("SELECT id, nombre, genero, estado_civil, fecha_nacimiento, telefono, direccion, email, validado, observacion FROM persona", &[])? {
        let personas = Persona {
            _id: row.get(0),
            _nombre: row.get(1),
            _genero: row.get(2),
            _estado_civil: row.get(3),
            _fecha_nacimiento: row.get(4),
            _telefono: row.get(5),
            _direccion: row.get(6),
            _email: row.get(7),
            _valido: row.get(8),
            _observaciones: row.get(9)
        };
        println!(
                "{0: <6}| {1: <11}| {2: <17} | {3: <6},{4: <12} | {5: <10} | {6: <11} | {7: <17} | {8: <17} ",
                "Valido","ID", "Nombre", "Género", "Estado C", "Fecha Nac.","Teléfono","Dirección","Email"
            );
        println!("{0: <6}| {1: <11}| {2: <17} | {3: <6},{4: <12} | {5: <10} | {6: <11} | {7: <17} | {8: <17}", personas._valido, personas._id, personas._nombre, personas._genero, personas._estado_civil, personas._fecha_nacimiento, personas._telefono, personas._direccion, personas._email );
        println!("{}", " " );
        println!("{}", personas._observaciones );
        println!("{}", " " );

    }



       client.batch_execute("
       DROP TABLE persona;
    ")?;

    Ok(())
}
