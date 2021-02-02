#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


#[cfg(test)] mod tests;
use postgres::{Client, NoTls, Error};
use std::io;
use std::fs::File;
use std::path::Path;
use std::fs;
use rocket::Data;
use rocket::response::{content, Debug};
use rocket::http::Status;


const HOST: &str = "http://localhost:3000";


struct persona {
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



fn es_numero(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}
fn n_dig(num: i64) -> i32 {
	if num < 10 {
	return 1;
	}else if num < 100{
	return 2;
	}else if num < 1000{
	return 3;
	}else if num < 10000{
	return 4;
	}else if num < 100000{
	return 5;
	}else if num < 1000000{
	return 6;
	}else if num < 10000000{
	return 7;
	}else if num < 100000000{
	return 8;
	}else if num < 1000000000{
	return 9;
	}else if num < 10000000000{
	return 10;
}else {
    return 0;
};

}
fn iden(num: String) -> (i64, String) {
    if num.is_empty(){
        return (0, "Identificacion no dada, ".to_string())
    }
    if es_numero(num.clone()){
            let a: i64 = num.parse().unwrap();

          if n_dig(a) == 10 {
                 if a/100000000 > 24 && a/100000000 != 30 && a/100000000 != 50 && a/100000000 != 80 {

                             return (a, "Id no es valido por número inicial equivocado, ".to_string())
                          } else
                          {return (a,"valido".to_string())}
                      }
            else{
                return (a, "Id no valido por cantidad de digitos diferente a 10, ".to_string())
             }

	} else {
        let b: i64 = 0;
        return (b,"Id no válida porque no es un numero, ".to_string())
    };

}

//fn name(_nombre: String) -> (String, String){
fn name(_nombre: String)-> (String, String){
    if _nombre.is_empty(){
        return ("-".to_string(), "Nombre no dado, ".to_string())
    }

    let mut i = 0;
    let mut _nombre1= _nombre.clone();
    for token in _nombre.split_whitespace(){
        for n in token.chars(){
            if n.is_alphanumeric() {
                if n.is_numeric(){
                        return (_nombre,"Nombre no valido por tener numeros, ".to_string())
                }else
                {
                    if n.is_uppercase(){

                        if n == 'Ó'  {
                            _nombre1=_nombre.replace("Ó","O");
                            println!("{:?}", _nombre1);
                        } else if n == 'Á' {
                             _nombre1=_nombre.replace("Á","A");
                        } else if n == 'É' {
                             _nombre1=_nombre.replace("É","E");
                        } else if n == 'Í'{
                             _nombre1=_nombre.replace("Í","I");
                        }else if n == 'Ú'{
                             _nombre1=_nombre.replace("Ú","U");
                        }else{}
                    }else {

                        return (_nombre,"Nombre no valido por minúscula, ".to_string())
                    }

                }

            }else{
                return (_nombre,"Nombre no valido por caracteres especiales, ".to_string())
            }
  }
        i+=1;
}
if i < 2{
    return (_nombre,"Nombre sin apellido".to_string())
}else{
    return (_nombre1,"valido".to_string())
}

}

fn genero(_genero: String)-> (String, String) {
    if _genero.is_empty(){
        return ("-".to_string(), "género no dado, ".to_string())
    }

 if _genero == "F" || _genero == "M" || _genero == "NULL"{
     return (_genero,"valido".to_string())
 }else{
     return (_genero,"Error en género, sólo F, M y NULL son validos, ".to_string())

 }

}

fn e_civil(_estado_civil: String) -> (String, String) {
    if _estado_civil.is_empty(){
        return ("-".to_string(), "Estado civil no dado, ".to_string())
    }


 if _estado_civil == "SOLTERO" || _estado_civil == "CASADO" || _estado_civil == "DIVORCIADO" || _estado_civil == "VIUDO" || _estado_civil == "EN UNION DE HECHO" || _estado_civil == "NULL"{
     return (_estado_civil,"valido".to_string())
 }else if _estado_civil == "UNION LIBRE" || _estado_civil == "SEPARADO"{
     let mut _estado_civil = "".to_string();
     return (_estado_civil,"Estado civil inactivo, ".to_string());

 }else{
     return (_estado_civil,"Estado no valido, ".to_string())

 }

}


fn fecha(_fecha_nacimiento: String)-> (String, String){
    if _fecha_nacimiento.is_empty(){
        return ("-".to_string(), "Fecha no dada, ".to_string())
    }

    let mut i = 0;
    for token in _fecha_nacimiento.split("-"){
        if i == 0{
            if token.chars().all(char::is_numeric){
                if token.parse::<i32>().unwrap() > 1926 && token.parse::<i32>().unwrap() <2013{

                }else{
                    return (_fecha_nacimiento,"fuera del limite de edad, ".to_string())
                }
            }else{
                return (_fecha_nacimiento,"año erroneo, ".to_string())
            }
        //for n in token.chars(){
    //            if n.is_numeric(){
    //                    return (_nombre,"Nombre no valido por numero".to_string())
    //            }else
    //            {            }
    }else if i == 1{
        if token.chars().all(char::is_numeric){
            if token.parse::<i32>().unwrap() >0 && token.parse::<i32>().unwrap() <13{

            }else{
                return (_fecha_nacimiento,"error en el mes, ".to_string())
            }
        }else{
            return (_fecha_nacimiento,"mes erroneo, ".to_string())
        }

    } else if i == 2{
        if token.chars().all(char::is_numeric){
            if token.parse::<i32>().unwrap() >0 && token.parse::<i32>().unwrap() <32{

            }else{
                return (_fecha_nacimiento,"error en el dia, ".to_string())
            }
        }else{
            return (_fecha_nacimiento,"día fuera de los límites, ".to_string())
        }

    }else if i==3{
        return (_fecha_nacimiento,"formato erroneo (YYYY/MM/DD), ".to_string())
    }
        i+=1;
}

if i != 3 {return(_fecha_nacimiento,"formato erroneo(YYYY/MM/DD)".to_string())}
else{
  return(_fecha_nacimiento,"valido".to_string())}
}



fn tel(_telefono: String) -> (String, String) {
    if _telefono.is_empty(){
        return ("-".to_string(), "telefono no dado, ".to_string())
    }

    if es_numero(_telefono.clone()){
                //let a: i64 = _telefono.parse().unwrap();

              if _telefono.chars().count() == 10 {
                     if _telefono.chars().nth(0).unwrap() == '0' && _telefono.chars().nth(1).unwrap() == '9' {

                                 return (_telefono, "valido".to_string())
                              } else
                              {return (_telefono,"Teléfono debe iniciar en 09, ".to_string())}
                          }
                else{
                    return (_telefono, "no valido por la cantidad de digitos no es 10, ".to_string())
                 }

    	} else {
            return (_telefono,"contiene carácteres no validos, ".to_string())
        };



}



fn direccion(_direccion: String)-> (String, String){
    if _direccion.is_empty(){
        return ("-".to_string(), "direccion no dada, ".to_string())
    }

    let mut i = 0;
    for token in _direccion.split_whitespace(){
        i+=1;
}
if i < 2{
    return (_direccion,"direccion no valida, mínimo 2 palabras, ".to_string())
}else{
    return (_direccion,"valido".to_string())
}

}


fn email(_email: String)-> (String, String){
    if _email.is_empty(){
        return ("-".to_string(), "Correo no dado".to_string())
    }
    let mut i = 0;
if _email.contains(char::is_whitespace){
    return (_email,"Email no valido por espacio en el email ".to_string())
}

    for token in _email.split('@'){


        for n in token.chars(){
            if i == 0{
                let c = token.chars().count();

                if token.chars().nth(c-1).unwrap() == '.'
                {

                    return (_email,"Email no valido por punto antes de @".to_string());

            }

            if n.is_alphanumeric() || n == '-' || n == '.' || n == '_' {

            }else{
                return (_email,"Email no valido por uso de caracteres especiales".to_string())
            }
        }else if i == 1{
            if token.chars().nth(0).unwrap() == '.'
            {

                return (_email,"Email no valido por punto luego de @".to_string());

            }
            //for token in _email.split('.'){
            if token.chars().count() <2 && token.chars().count()>6 {
                return (_email,"Email no valido por cantidad de letras".to_string())

            }
        //}
        }

  }

        i+=1;
}
if i < 2{
    return (_email,"falta el @".to_string())
}else{
    return (_email,"valido".to_string())
}

}


fn valido(_ob_nombre: String, _ob_tel: String, _ob_email: String)-> (i32){

    if _ob_nombre == "valido" && _ob_tel == "valido" && _ob_email == "valido" {
        return 1;
    }else{
        return 0;
    }

    }

fn observacion(_obid: String, _obnombre: String, _obgenero: String, _obestado: String,_obfecha: String, _obtel: String, _obdir: String,_obemail: String)-> (String){
    let mut _obf: String = "Observaciones: ".to_owned();
    if _obid != "valido"{
        _obf.push_str(&_obid.to_owned());
    }
    if _obnombre != "valido"{
        _obf.push_str(&_obnombre.to_owned());
    }
    if _obgenero != "valido"{
        _obf.push_str(&_obgenero.to_owned());
    }
    if _obestado != "valido"{
        _obf.push_str(&_obestado.to_owned());
    }
    if _obfecha != "valido"{
        _obf.push_str(&_obfecha.to_owned());
    }
    if _obtel != "valido"{
        _obf.push_str(&_obtel.to_owned());
    }
    if _obdir != "valido"{
        _obf.push_str(&_obdir.to_owned());
    }
    if _obemail != "valido"{
        _obf.push_str(&_obemail.to_owned());
    }



    println!("{:?}", _obf);
    //if _ob_id != "valido"{

    //}
    //if _ob_id != "valido"{

    //}
    //if _ob_id != "valido"{

    //}
    return _obf
}






#[post("/upload" )]
fn upload() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:avila2017@localhost:5432/postgres", NoTls)?;
    // client.batch_execute("
    //     DROP TABLE persona;
    // ")?;


     client.batch_execute("
         CREATE TABLE IF NOT EXISTS persona (
             id              BIGINT PRIMARY KEY,
             nombre            VARCHAR,
             genero         VARCHAR,
 	    estado_civil	VARCHAR,
 	    fecha_nacimiento	VARCHAR,
 	    telefono		VARCHAR,
 	    direccion 		VARCHAR,
  	    email		VARCHAR,
 	    validado        SERIAL,
 	    observacion		VARCHAR
             )
     ")?;
     let mut rdr = csv::ReaderBuilder::new()
         .has_headers(true)
         .delimiter(b';')
         .from_path("upload/data.csv").expect("Unable to open");
     //let mut rdr = csv::ReaderBuilder::new()
    //     .delimiter(b';')
    //     .from_reader(io::stdin());
         for result in rdr.records() {
             let record = result.expect("a CSV record");

             //IDENTIFICACION
             let mut _id: i64;
             let mut _ob_id: String;
             let (_id, _ob_id) = iden(record[0].to_string());
             println!("{} {:?}","ID:", _id);


             //Nombre
     	      let mut _nombre = &record[1];
               let mut _ob_nombre: String;
               let (_nombre, _ob_nombre) = name(_nombre.to_string());
               println!("{} {:?}","NOMBRE: ", _nombre);


              //GENERO
     	     let mut _genero = &record[2];
              let mut _ob_genero: String;

              let (_genero, _ob_genero) = genero(_genero.to_string());
              println!("{} {:?}","GENERO: ", _genero);


              //ESTADOCIVIL
     	     let mut _estado_civil = &record[3];
              let mut _ob_estado: String;
              let (_estado_civil, _ob_estado) = e_civil(_estado_civil.to_string());
              println!("{} {:?}","ESTADO C: ", _estado_civil);

              //fecha de nacimiento
         //let _date = NaiveDate::parse_from_str(&record[4], "%Y-%m-%d");
     	let mut _fecha_nacimiento = &record[4];
         let mut _ob_fecha: String;
         let (_fecha_nacimiento, _ob_fecha) = fecha(_fecha_nacimiento.to_string());
         println!("{} {:?}","FECHA NACIMIENTO: ", _fecha_nacimiento);


         //TELEFONO
     	let mut _telefono = &record[5];
         let mut _ob_tel: String;
     	//let _telefono: i64 = my_stringtel.parse().unwrap();
         let (_telefono, _ob_tel) = tel(_telefono.to_string());
         println!("{} {:?}","TELEFONO: ", _telefono);

         //DIRECCION
     	let mut _direccion = &record[6];
         let mut _ob_dir: String;
         let (_direccion, _ob_dir) = direccion(_direccion.to_string());
         println!("{} {:?}","DIRECCION: ", _direccion);

         //EMAIL
     	let mut _email = &record[7].to_string();
         let mut _ob_email: String;
         let (_email, _ob_email) = email(_email.to_string());
         println!("{} {:?}","EMAIL: ", _email);

         let mut _valido : i32 =valido(_ob_nombre.clone(), _ob_tel.clone(),_ob_email.clone());
         println!("{} {:?}","Valido: " ,_valido);

         let mut _observacion : String =observacion(_ob_id,_ob_nombre.clone(),_ob_genero,_ob_estado,_ob_fecha,_ob_tel.clone(),_ob_dir,_ob_email.clone());

             client.execute(
                 "INSERT INTO persona (id, nombre, genero, estado_civil, fecha_nacimiento, telefono, direccion, email, validado, observacion) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                 &[&_id, &_nombre, &_genero, &_estado_civil, &_fecha_nacimiento, &_telefono, &_direccion, &_email, &_valido, &_observacion],
             )?;
         println!("{}"," " );

      	}


        let data = "200 OK";
        fs::write("msg", data).expect("Unable to write file");
        //let ok = Status::Ok;
        //assert_eq!(ok.to_string(), "200 Not Found".to_string());
    Ok(())
}

#[get("/upload")]
fn retrieve() -> Option<content::Plain<File>> {
    let filename = "msg";
    File::open(&filename).map(|f| content::Plain(f)).ok()

}

#[get("/")]
fn index() -> &'static str {
fs::remove_file("msg");
    "
    Bienvenido!
    En este servicio puedes hacer la migración de tus datos desde un archivo CSV a una base de datos en PostgreSQL.
    En http://localhost:3000/upload puedes ver si los datos ya fueron cargados correctamente.

    Para cargarlos guarda el archivo en la carpeta upload y luego ejecuta dentro del proyecto el comando:

                        curl --data-raw @upload/data.csv http://localhost:3000/upload

    En la terminal se mostrarán los valores almacenados para cada persona y se puede verificar haciendo una búsqueda en
    base de datos
                        cargo run --bin query
    "
}


#[get("/datos")]
fn datos()-> String{

 return "correcto".to_string();

}



fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index,retrieve, upload, datos])
}


fn main() {

    rocket().launch();
}
