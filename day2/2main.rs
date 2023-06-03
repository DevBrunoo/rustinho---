const SECONDS_IN_MINUTE: u32 = 60; // Definindo uma constante, ela pode ser definida no inicio do escopo, dentro de uma funcao ou no contexto // Aqui esamos dizendo que um segundo tem 60 segundo armazenados em u32 bits inteiros positivos 
const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

fn main(){
    
    let total = 30;
    let total_em_segundos = total * SECONDS_IN_HOUR;
    println!("O Bruno trabalhou {} segundos, no seu novo projeto", total_em_segundos);
}