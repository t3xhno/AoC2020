// Ucitavanje biblioteka
use std::collections::HashMap; // Ovo je struktura mapa iz standardne biblioteke
// Naredne dve biblioteke su za citanje ulaza kao buffer
// BufReader je bukvalno struktura koja cita
// BufRead sadrzi funkciju lines() na strukturi BufReader
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main () {
    // Svaka funkcija koja moze da fail-uje iz bilo kog razloga,
    // u ovom slucaju File::open (recimo, fajl ne postoji),
    // vraca objekat koji se zove Result. Result sadrzi
    // return vrednost te funkcije ili error
    // Mogu da se handle-uju na vise nacina, tipa sa match
    // Najjednostavniji nacin, kada znamo da nece da baci gresku, jeste
    // unwrap(), koji samo vrati vrednost. Problem sa unwrap je sto
    // ne handle-uje gresku, tako da program umre ako dodje do greske.
    // Zato se koristi samo u ovim slucajevima kada bas znamo
    // da nece doci do greske.
    let reader = BufReader::new(File::open("dataset.txt").unwrap());
    // mut kaze da je "promenljiva" mutabilna (nije konstantna)
    // : HashMap<u16, u8> kaze da je tip promenljive has_map
    // HashMap-a ciji su kljucevi u16 (unsigned int na 16 bit-a),
    // a vrednosti u8 (unsigned int 8 bit-a)
    // Ovde se inicijalizuje praznom HashMap-om putem ugradjene funkcije
    let mut hash_map: HashMap<u16, u8> = HashMap::new();

    // reader.lines() znaci cita liniju po liniju iz input-a
    // enumerate() sluzi da umesto da se vrati samo linija - vrati
    // se tuple koji sadrzi indeks i liniju. Ovo se cesto koristi
    // tokom iteracije
    reader.lines().enumerate()
        // .map je standardno mapiranje kao i u bilo kom drugom jeziku
        // |(i, line)| je element koji se uzima pri mapiranju
        // Ovde je tuple zbog prethodnog enumerate()
        // i as u8 cast-uje i koji je usize tipa u tip u8
        // Ovakvo cast-ovanje je moguce izmedju svih primitivnih tipova
        // Razlog zasto cast-ujem je zato sto sam index definisao kao u8
        // pri inicijalizaciji HasMap-e. Moglo je vrv da bude od samog
        // pocetka usize sve
        // line.unwrap() - unwrap zato sto line vraca result, posto je moguce
        // da dodje do greske u citanju linije (ne postoji?)
        // .parse() parsira vrednost po default-u u i32
        // (i32 je generalno default implicirani tip za sve numericke promenljive)
        // parse::<u16>() pokusava da cast-uje string u u16, sto vraca
        // Result, zato sto moze da fail-uje (string ne sadrzi samo numericke karaktere),
        // pa zato na kraju stoji opet unwrap()
        .map(|(i, line)| (i as u8, line.unwrap().parse::<u16>().unwrap()))
        // try_for_each je kao for_each, sa malom razlikom
        // for_each uvek ide kroz sve elemente, ne moze se prekinuti
        // try_for_each ide kroz sve elemente, medjutim prestane kada naleti na true
        // Some() sluzi da bi mogao da postoji slucaj gde je nesto null, odnosto none,
        // odnosno ne postoji
        // Ovde Some sluzi za proveru da li postoji vrednost koja zadovoljava
        // trazeni uslov, kako bi mogao da se handle-uje i slucaj kada to nije tacno
        .try_for_each(|(i, line)| Some({
            // match je slicno switch-u
            // & je referenciranje, * dereferenciranje
            // & se koristi zato sto je to zahtevano u samoj definiciji contains_key,
            // a razlog je da se ne bi prosledio ownership, te da ne bi doslo prerano do
            // dealokacije memorije kojoj pristupa
            //
            // Ovo je glavna logika programa sada
            // Pocinje da parsira input direktno kako ga cita, liniju po liniju
            // Za svaku liniju, proverava da li komplement njene numericke vrednosti
            // Postoji kao kljuc u HasMap-i. Ako postoji, vraca true, ispisuje
            // proizvod numericke vrednosti te linije i njenog komplementarnog broja do 2020
            // Zbog vracanja true i try_for_each, tu se zavrsava citanje input-a,
            // sto znaci da ukoliko takva dva broja postoje u input-u, 
            // on nikad ne bude procitan ceo. Ovo je optimizacija samo
            // U slucaju da taj uslov nije zadovoljen za trenutnu liniju,
            // njena numericka vrednost se ubacuje u HashMap-u kao kljuc,
            // a vrednost za taj kljuc je njen indeks (ovo u sustini nije vazno sta je vrednost,
            // samo se kljucevi proveravaju)
            match hash_map.contains_key(&(2020 - line)) && *hash_map.get(&(2020 - line)).unwrap() != i {
                true => {
                    // "{}" - klasicno formatiranje output-a
                    println!("{}", line * (2020 - line));
                },
                // _ oznacava bilo koju vrednost (kao * u nekim regex-ima, wildcard)
                // U principu, ova "ruka" se izvrsava ako uslov nije true
                _ => {
                    // Ubacivanje linije u HashMap-u (cast-ovana je gore u map funkciji u u16
                    // sa onim parse::<u16>()) i indeksa kao vrednost za kljuc (ovo je
                    // irelevantno, kao sto sam vec naveo)
                    hash_map.insert(line, i);
                }
            }
        }));
}