//Hacer una aplicacion de un banco en la cual se pueda ver el dinero, retirar y depositar.
use std::io;
fn bank(mut dinero: i32){
    println!("Banco");
    loop {
        println!("1.Depositar");
        println!("2.Retirar");
        println!("3.Ver Dinero");
        println!("4.Salir");
        println!("Seleccione una opcion: ");
        let mut opcion = String::new();
        io::stdin()
            .read_line(&mut opcion)
            .expect("Error");
        let option: i32 = opcion.trim().parse().expect("Please enter a valid number");
        if option == 4{
            println!("Adios :3");
            break;
        } // If Exit

        if option == 1{
            println!("¿Cuanto dinero quiere depositar?: ");
            let mut depositar = String::new();
            io::stdin()
                .read_line(&mut depositar)
                .expect("Error");
            let deposito: i32 = depositar.trim().parse().expect("Please enter a valid number");
            dinero += deposito;
            println!("Saldo actual: {}", dinero);
        } else if option == 2{
            println!("¿Cuanto dinero quiere retirar?: ");
            let mut retirar = String::new();
            io::stdin()
                .read_line(&mut retirar)
                .expect("Error");
            let retiro: i32 = retirar.trim().parse().expect("Please enter a valid number");
            if retiro > dinero{
                println!("Saldo Insuficiente");
            }else{
            dinero-=retiro;
            println!("Saldo actual: {}", dinero);
            }//End If Retiro
        } else if option == 3 {
            println!("Saldo actual: {}", dinero);
        } // End if
    } // End Loop
}//End fn


fn main(){
    bank(3200);
}
