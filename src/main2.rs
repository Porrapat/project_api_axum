// main.rs
use async_trait::async_trait;
use tokio;

#[async_trait]
trait Fetcher {
    async fn fetch(&self, url: &str) -> String;
}

struct MyFetcher;

#[async_trait]
impl Fetcher for MyFetcher {
    async fn fetch(&self, url: &str) -> String {
        // ตัวอย่าง: ทำเหมือนโหลดข้อมูล
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        format!("✅ Fetched from {}", url)
    }
}

#[tokio::main]
async fn main() {
    let fetcher = MyFetcher;

    println!("====== Start ======");

    let result = fetcher.fetch("https://example.com").await;
    println!("{result}");
    let result2 = fetcher.fetch("https://example.com").await;
    println!("{result2}");
    let result3 = fetcher.fetch("https://example.com").await;
    println!("{result3}");
    let result4 = fetcher.fetch("https://example.com").await;
    println!("{result4}");
    let result5 = fetcher.fetch("https://example.com").await;
    println!("{result5}");

    // // สมมุติมี let fetcher = MyFetcher; อยู่แล้ว
    // let f1 = fetcher.fetch("https://example.com");
    // let f2 = fetcher.fetch("https://example.com");
    // let f3 = fetcher.fetch("https://example.com");
    // let f4 = fetcher.fetch("https://example.com");
    // let f5 = fetcher.fetch("https://example.com");

    // // รอทั้งหมดพร้อมกัน
    // let (r1, r2, r3, r4, r5) = tokio::join!(f1, f2, f3, f4, f5);

    // println!("{r1}");
    // println!("{r2}");
    // println!("{r3}");
    // println!("{r4}");
    // println!("{r5}");

}
