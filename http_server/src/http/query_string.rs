use std::collections::HashMap;

// Example of query
//   a=1&b=2&d=&e====&d=7&d=abc
// Note that d can have multiple value => d is an array.

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

// A dynamic array allocated on the heap is called Vec
// in rust...
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {

    fn from(s:&'buf str) -> Self {
        let mut data: HashMap<&'buf str, Value<'buf>> = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i+1..];
            }

            // At this point we need to check if the key is a new one or not
            // And if it is not a new one there are two cases. It has a Single
            // value so we need to create a Multiple one and it it is already
            // a Multiple value we just need to add the new value.
            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![prev_val, val]);
                    },
                    Value::Multiple(vec) => {vec.push(val)},
                })
                .or_insert(Value::Single(val));
        }

        QueryString {
            data,
        }
    }
}
