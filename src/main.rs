fn main() {
    println!("Hello, world!");
}
use std::collections::HashMap;

struct Hesap {
    hesap_numarasi: String,
    bakiye: f64,
}

impl Hesap {
    fn yeni(hesap_numarasi: String, bakiye: f64) -> Self {
        Hesap {
            hesap_numarasi,
            bakiye,
        }
    }
    fn bakiye_goster(&self) {
        println!("Hesap {} Bakiyesi: {:.2} TL", self.hesap_numarasi, self.bakiye);
    }

    fn para_yatir(&mut self, miktar: f64) {
        self.bakiye += miktar;
        println!("{} TL yatırıldı. Yeni bakiye: {:.2} TL", miktar, self.bakiye);
    }

    fn para_cek(&mut self, miktar: f64) {
        if self.bakiye >= miktar {
            self.bakiye -= miktar;
            println!("{} TL çekildi. Yeni bakiye: {:.2} TL", miktar, self.bakiye);
        } else {
            println!("Yetersiz bakiye!");
        }
    }
}

struct Banka {
    banka_adi: String,
    hesaplar: HashMap<String, Hesap>,
}

impl Banka {
    fn yeni(banka_adi: String) -> Self {
        Banka {
            banka_adi,
            hesaplar: HashMap::new(),
        }
    }
}

struct Kullanici {
    ad: String,
    hesaplar: Vec<Hesap>,
}

impl Kullanici {
    fn yeni(ad: String) -> Self {
        Kullanici {
            ad,
            hesaplar: Vec::new(),
        }
    }

    fn hesap_ekle(&mut self, hesap: Hesap) {
        self.hesaplar.push(hesap);
    }
}

fn para_transferi(gonderen: &mut Hesap, alici: &mut Hesap, miktar: f64) {
    if gonderen.bakiye >= miktar {
        gonderen.bakiye -= miktar;
        alici.bakiye += miktar;
        println!("Transfer başarılı! {} TL {} numaralı hesaptan {} numaralı hesaba aktarıldı.", miktar, gonderen.hesap_numarasi, alici.hesap_numarasi);
    } else {
        println!("Yetersiz bakiye, transfer başarısız!");
    }
}

fn para_talep_et(alici: &mut Hesap, gonderen: &mut Hesap, miktar: f64) {
    if gonderen.bakiye >= miktar {
        println!("Talep edilen {} TL gönderiliyor...", miktar);
        para_transferi(gonderen, alici, miktar);
    } else {
        println!("Göndericinin yeterli bakiyesi yok, para talebi reddedildi!");
    }
}

fn main() {
    let banka1 = Banka::yeni("Ziraat Bankası".to_string());
    let banka2 = Banka::yeni("Garanti Bankası".to_string());
    let banka3 = Banka::yeni("İş Bankası".to_string());
    let banka4 = Banka::yeni("Yapı Kredi Bankası".to_string());
    
    let mut kullanici1 = Kullanici::yeni("Ceren".to_string());
    let mut kullanici2 = Kullanici::yeni("Güliz".to_string());
    let mut kullanici3 = Kullanici::yeni("Deniz".to_string());
    let mut kullanici4 = Kullanici::yeni("Murat".to_string());
    
    let mut hesap1 = Hesap::yeni("1234".to_string(), 1000.0);
    let mut hesap2 = Hesap::yeni("987".to_string(), 2500.0);
    let mut hesap3 = Hesap::yeni("4567".to_string(), 5000.0);
    let mut hesap4 = Hesap::yeni("7890".to_string(), 750.0);
    let mut hesap5 = Hesap::yeni("5678".to_string(), 3200.0);
    let mut hesap6 = Hesap::yeni("4321".to_string(), 4100.0);
    
    kullanici1.hesap_ekle(hesap1);
    kullanici1.hesap_ekle(hesap5);
    kullanici2.hesap_ekle(hesap2);
    kullanici3.hesap_ekle(hesap3);
    kullanici3.hesap_ekle(hesap6);
    kullanici4.hesap_ekle(hesap4);
    
    for hesap in &kullanici1.hesaplar {
        hesap.bakiye_goster();
    }
    for hesap in &kullanici2.hesaplar {
        hesap.bakiye_goster();
    }
    for hesap in &kullanici3.hesaplar {
        hesap.bakiye_goster();
    }
    for hesap in &kullanici4.hesaplar {
        hesap.bakiye_goster();
    }
    
    para_transferi(&mut kullanici1.hesaplar[0], &mut kullanici2.hesaplar[0], 300.0);
    
    for hesap in &kullanici1.hesaplar {
        hesap.bakiye_goster();
    }
    for hesap in &kullanici2.hesaplar {
        hesap.bakiye_goster();
    }
    for hesap in &kullanici3.hesaplar {
        hesap.bakiye_goster();
    }
    for hesap in &kullanici4.hesaplar {
        hesap.bakiye_goster();
    }
    
    para_talep_et(&mut kullanici2.hesaplar[0], &mut kullanici1.hesaplar[0], 200.0);
    
    for hesap in &kullanici1.hesaplar {
        hesap.bakiye_goster();
    }
    for hesap in &kullanici2.hesaplar {
        hesap.bakiye_goster();
    }
}
