use rand::Rng;
use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::{
    net::TcpStream,
    sync::mpsc,
    task,
    time::{interval, sleep, timeout},
};

const BATCH_SIZE: usize = 50_000;
const TELEGRAM_BOT_TOKEN: &str = "TON_TOKEN_ICI";
const TELEGRAM_CHAT_ID: &str = "TON_CHAT_ID_ICI";
const SENT_IPS_FILE: &str = "sent_ips.txt";
const WORKER_COUNT: usize = 8;
const TCP_TIMEOUT_MS: u64 = 200;

fn generate_random_ip() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "{}.{}.{}.{}",
        rng.gen_range(1..=254),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(1..=254)
    )
}

fn load_sent_ips(path: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            set.insert(line);
        }
    }
    set
}

fn save_ips_to_file(ips: &[String], filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    for ip in ips {
        writeln!(writer, "{}", ip)?;
    }
    Ok(())
}

fn append_sent_ips(path: &str, ips: &[String]) -> std::io::Result<()> {
    let file = OpenOptions::new().create(true).append(true).open(path)?;
    let mut writer = BufWriter::new(file);
    for ip in ips {
        writeln!(writer, "{}", ip)?;
    }
    Ok(())
}

async fn send_file_to_telegram(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.telegram.org/bot{}/sendDocument",
        TELEGRAM_BOT_TOKEN
    );

    let form = reqwest::multipart::Form::new()
        .text("chat_id", TELEGRAM_CHAT_ID)
        .file("document", filename)?;

    let client = reqwest::Client::new();
    let res = client.post(&url).multipart(form).send().await?;

    if res.status().is_success() {
        println!("✅ Fichier envoyé !");
    } else {
        eprintln!("❌ Erreur Telegram : {}", res.text().await?);
    }

    Ok(())
}

async fn is_ip_valid(ip: &str) -> bool {
    let socket: SocketAddr = format!("{}:80", ip).parse().unwrap_or_else(|_| return false);
    timeout(Duration::from_millis(TCP_TIMEOUT_MS), TcpStream::connect(socket))
        .await
        .is_ok()
}

#[tokio::main]
async fn main() {
    println!("🚀 Générateur d'IP avec test réseau et log de vitesse lancé...");

    let sent_ips = Arc::new(Mutex::new(load_sent_ips(SENT_IPS_FILE)));
    let (tx, mut rx) = mpsc::channel::<String>(BATCH_SIZE * 2);

    let generated_counter = Arc::new(Mutex::new(0usize));
    let valid_counter = Arc::new(Mutex::new(0usize));

    // Thread log de vitesse
    {
        let gen = Arc::clone(&generated_counter);
        let valid = Arc::clone(&valid_counter);
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(10));
            loop {
                ticker.tick().await;
                let g = *gen.lock().unwrap();
                let v = *valid.lock().unwrap();
                println!("📊 Stats - Générées: {} | Valides envoyées: {} | ~{} IP/min", g, v, v * 6);
            }
        });
    }

    // Générateurs concurrents
    for _ in 0..WORKER_COUNT {
        let tx = tx.clone();
        let sent_ips = Arc::clone(&sent_ips);
        let counter = Arc::clone(&generated_counter);

        task::spawn(async move {
            loop {
                let ip = generate_random_ip();
                *counter.lock().unwrap() += 1;

                {
                    let sent = sent_ips.lock().unwrap();
                    if sent.contains(&ip) {
                        continue;
                    }
                }

                if is_ip_valid(&ip).await {
                    if tx.send(ip).await.is_err() {
                        break;
                    }
                }
            }
        });
    }

    // Collecte
    loop {
        let mut valid_ips = Vec::with_capacity(BATCH_SIZE);

        while valid_ips.len() < BATCH_SIZE {
            if let Some(ip) = rx.recv().await {
                let mut sent = sent_ips.lock().unwrap();
                if !sent.contains(&ip) {
                    sent.insert(ip.clone());
                    valid_ips.push(ip);
                    *valid_counter.lock().unwrap() += 1;
                }
            }
        }

        let filename = "ips_batch.txt";
        if save_ips_to_file(&valid_ips, filename).is_ok() {
            let _ = send_file_to_telegram(filename).await;
            let _ = append_sent_ips(SENT_IPS_FILE, &valid_ips);
            println!("📦 50 000 IP envoyé !");
        }

        sleep(Duration::from_secs(1)).await;
    }
}
