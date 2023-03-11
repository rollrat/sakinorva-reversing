# sakinorva-reversing

Sakinorva Cognitive Function Test Reversing Project

https://sakinorva.net/functions?lang=kr

## Plan

```
1. Correlation between questions and mbti features
2. A genetic algorithm based question selector to create specific mbti types
```

## Example

```rs
#[tokio::main]
async fn main() {
    let mbti = get_functions_from_features(Features {
        // range is 1 ~ 12
        ti: 6,
        te: 6,
        si: 6,
        se: 6,
        ni: 6,
        ne: 6,
        fi: 6,
        fe: 6,
    })
    .await;

    println!("{:#?}", mbti.parse_features());
    println!("{}", mbti.parse_myers_letter_type());
}
```

```
{
    "Ti": 35.0,
    "Fi": 35.0,
    "Se": 35.0,
    "Ni": 35.8,
    "Te": 35.0,
    "Si": 35.0,
    "Ne": 34.2,
    "Fe": 35.0,
}
INTP
```

## Irregulation Question

```
q47, 당신은 카리스마를 내뿜고, 남들에게 매력적으로 보인다. {
    "Ne": 22.2,
    "Fi": 23.0,
    "Fe": 25.0,
    "Ti": 23.0,
    "Ni": 23.8,
    "Si": 23.0,
    "Te": 23.0,
    "Se": 23.0,
}
q69, 당신은 남들에게 예민하고 우울한 사람으로 보이기도 한다. {
    "Te": 23.0,
    "Se": 23.0,
    "Ne": 24.2,
    "Ni": 23.0,
    "Si": 23.0,
    "Ti": 23.0,
    "Fe": 23.0,
    "Fi": 25.0,
}
```
