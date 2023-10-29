use anyhow::Context;
use rand;

fn main() {
    let result = fun2();

    match result {
        Ok(result) => {
            println!("Result: {}", result);
        }
        Err(error) => {
            // To wypisze cały chain błędów jakie wystąpiły
            error.chain().for_each(|cause| {
                println!("Cause: {}", cause);
            });

            // To wypisze ostatni błąd
            println!("Cause: {}", error);
        }
    }
}

fn fun2() -> anyhow::Result<i32> {
    fun3().with_context(|| "Context from fun 2")?; // with_contex takes closure

    let random = rand::random::<i32>();

    if random & 2 == 0 {
        Ok(random)
    } else {
        Err(anyhow::anyhow!("Fun: 2 random number too large!"))
    }
}


fn fun3() -> anyhow::Result<i32> {
    fun4().context("Context from fun 3")?; // contex takes raw String

    let random = rand::random::<i32>();

    if random & 2 == 0 {
        Ok(random)
    } else {
        Err(anyhow::anyhow!("Fun: 3 random number too large!"))
    }
}

fn fun4() -> anyhow::Result<i32> {
    fun5().with_context(|| "Context from fun 4")?;

    let random = rand::random::<i32>();

    if random & 2 == 0 {
        Ok(random)
    } else {
        Err(anyhow::anyhow!("Fun: 4 random number too large!"))
    }
}

fn fun5() -> anyhow::Result<i32> {
    fun6().with_context(|| "Context from fun 5")?;

    let random = rand::random::<i32>();

    if random & 2 == 0 {
        Ok(random)
    } else {
        Err(anyhow::anyhow!("Fun: 5 random number too large!"))
    }
}


fn fun6() -> anyhow::Result<i32> {
    fun7().with_context(|| "Context from fun 6")?;

    let random = rand::random::<i32>();

    if random & 2 == 0 {
        Ok(random)
    } else {
        Err(anyhow::anyhow!("Fun: 6 random number too large!"))
    }
}


fn fun7() -> anyhow::Result<i32> {
    fun8().with_context(|| "Context from fun 7")?;

    let random = rand::random::<i32>();

    if random & 2 == 0 {
        Ok(random)
    } else {
        Err(anyhow::anyhow!("Fun: 7 random number too large!"))
    }
}


fn fun8() -> anyhow::Result<i32> {
    let random = rand::random::<i32>();

    if random & 2 == 0 {
        Ok(random)
    } else {
        Err(anyhow::anyhow!("Fun: 8 random number too large!"))
    }
}



