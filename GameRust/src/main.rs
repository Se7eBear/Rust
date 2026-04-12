fn main() {
    let mut heroi = Personagem::new("Alelo_Silva", 100,110,10);
    let mut monstro = Personagem::new("Glaubossauro", 100,10,10);


    println!("Um heroi vagando pela floresta se depara com uma situação incomum, ele encontra um monstro!! e decide que precisa elimina-lo");

    while heroi.checa_vivo() && monstro.checa_vivo(){
        println!("\nTurno do heroi por ter tomado atitude primeiro.");
        monstro.Receber_Dano(heroi.ataque);

        if !monstro.checa_vivo(){
            println!("O {} foi derrotado!", monstro.nome);
            break;
        }
        println!("\nTURNO DO MONSTRO, ele vai revidar...");
        heroi.Receber_Dano(monstro.ataque);

        if !heroi.checa_vivo(){
            println!("\nInfelizmente... o nosso querido Heroi {} Foi derrotado...", heroi.nome);
            break;
        }
        println!("\nvida atual do monstro {} \nvida atual do heroi {} ", monstro.vida, heroi.vida);
    }
    println!("Fim da batalha.");
}

#[derive(Debug)]
struct Personagem{
    nome:String,
    vida:i32,
    ataque:i32,
    defesa:i32,
}
impl Personagem{
    fn new(nome: &str, vida: i32, ataque: i32, defesa: i32) -> Self{
        Self {
            nome: nome.to_string(),
            vida,
            ataque,
            defesa,
        }
    }
    fn Receber_Dano(&mut self, F_ataque: i32){
        let dano_final = F_ataque - self.defesa;
        if dano_final > 0{
            self.vida -= dano_final;
            println!("{} Recebeu {} de dano", self.nome, dano_final);
        }else{
            println!("Errou o ataque! {} não recebeu danos.", self.nome);
        }
    }
    fn checa_vivo(&self) -> bool {
        self.vida > 0
    }
}