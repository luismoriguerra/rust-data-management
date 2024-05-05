use redis::Commands;

fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379/").expect("redis start failure");
    let mut conn = client.get_connection()?;

    let key = "counter";
    conn.set(key, 42)?;
    conn.incr(key, 1)?;
    conn.incr(key, 3)?;

    let count: i32 = conn.get(key)?;

    println!("{}", count);

    println!("##################");

    conn.set_nx("grettings", "hello")?;
    conn.set_nx("grettings", "world")?;
    let greeting: String = conn.get("grettings")?;

    println!("{}", greeting);

    println!("##################");

    let _: () = conn.zadd("people", "Christopher Columbus", 1451)?;
    let _: () = conn.zadd("people", "Isaac Newton", 1643)?;
    let _: () = conn.zadd("people", "Charlemagne", 747)?;
    let _: () = conn.zadd("people", "Napoleon Bonaparte", 1769)?;
    let _: () = conn.zadd("people", "Leonardo da Vinci", 1452)?;

    let people: Vec<String> = conn.zrange("people", 0, -1)?;
    println!("All people: {:?}", people);

    let before1452: Vec<String> = conn.zrangebyscore("people", 0, 1452)?;
    println!("Before 1452: {:?}", before1452);

    Ok(())
}
