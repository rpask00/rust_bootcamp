fn main() {
    { // example 1.
        let s1 = String::from("abc");
        {
            let s2 = String::from("asdf");
            let mut r = ret_ref(&s1, &s1);
            r = ret_ref(&s1, &s2);
            // to zadział, mimo że s1 i s1 nie mają takich samych lifetime'ów, ale minimalny lifetime jest równy lifetime'owi r
            println!("r = {}", r);
        }
    }

    { // example 2.
        let r: &String;
        let s1 = String::from("abc");
        {
            let s2 = String::from("asdf");
            let s3 = String::from("qwer");
            r = ret_ref2(&s1, &s2, &s3);
        }
        println!("r = {}", r);
    }
}


fn ret_ref<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}


fn ret_ref2<'a, 'b>(s1: &'a String, s2: &'a String, s3: &'b String) -> &'a String {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn ret_ref2<'a>(s1: &'a String, s2: &'a String, s3: &String) -> &'a String {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
