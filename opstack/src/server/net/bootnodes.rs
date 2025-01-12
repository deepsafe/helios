use std::str::FromStr;

use discv5::enr::{CombinedKey, Enr};

/// Default bootnodes to use. Currently consists of 2 Base bootnodes & 1 Op Mainnet bootnode.
pub fn bootnodes() -> Vec<Enr<CombinedKey>> {
    let bootnodes = [
        "enr:-J64QBbwPjPLZ6IOOToOLsSjtFUjjzN66qmBZdUexpO32Klrc458Q24kbty2PdRaLacHM5z-cZQr8mjeQu3pik6jPSOGAYYFIqBfgmlkgnY0gmlwhDaRWFWHb3BzdGFja4SzlAUAiXNlY3AyNTZrMaECmeSnJh7zjKrDSPoNMGXoopeDF4hhpj5I0OsQUUt4u8uDdGNwgiQGg3VkcIIkBg",
        "enr:-J64QAlTCDa188Hl1OGv5_2Kj2nWCsvxMVc_rEnLtw7RPFbOfqUOV6khXT_PH6cC603I2ynY31rSQ8sI9gLeJbfFGaWGAYYFIrpdgmlkgnY0gmlwhANWgzCHb3BzdGFja4SzlAUAiXNlY3AyNTZrMaECkySjcg-2v0uWAsFsZZu43qNHppGr2D5F913Qqs5jDCGDdGNwgiQGg3VkcIIkBg",
        "enr:-J24QGEzN4mJgLWNTUNwj7riVJ2ZjRLenOFccl2dbRFxHHOCCZx8SXWzgf-sLzrGs6QgqSFCvGXVgGPBkRkfOWlT1-iGAYe6Cu93gmlkgnY0gmlwhCJBEUSHb3BzdGFja4OkAwCJc2VjcDI1NmsxoQLuYIwaYOHg3CUQhCkS-RsSHmUd1b_x93-9yQ5ItS6udIN0Y3CCIyuDdWRwgiMr",

        // Base bootnodes
        "enr:-J24QNz9lbrKbN4iSmmjtnr7SjUMk4zB7f1krHZcTZx-JRKZd0kA2gjufUROD6T3sOWDVDnFJRvqBBo62zuF-hYCohOGAYiOoEyEgmlkgnY0gmlwhAPniryHb3BzdGFja4OFQgCJc2VjcDI1NmsxoQKNVFlCxh_B-716tTs-h1vMzZkSs1FTu_OYTNjgufplG4N0Y3CCJAaDdWRwgiQG",
        "enr:-J24QH-f1wt99sfpHy4c0QJM-NfmsIfmlLAMMcgZCUEgKG_BBYFc6FwYgaMJMQN5dsRBJApIok0jFn-9CS842lGpLmqGAYiOoDRAgmlkgnY0gmlwhLhIgb2Hb3BzdGFja4OFQgCJc2VjcDI1NmsxoQJ9FTIv8B9myn1MWaC_2lJ-sMoeCDkusCsk4BYHjjCq04N0Y3CCJAaDdWRwgiQG",
        "enr:-J24QDXyyxvQYsd0yfsN0cRr1lZ1N11zGTplMNlW4xNEc7LkPXh0NAJ9iSOVdRO95GPYAIc6xmyoCCG6_0JxdL3a0zaGAYiOoAjFgmlkgnY0gmlwhAPckbGHb3BzdGFja4OFQgCJc2VjcDI1NmsxoQJwoS7tzwxqXSyFL7g0JM-KWVbgvjfB8JA__T7yY_cYboN0Y3CCJAaDdWRwgiQG",
        "enr:-J24QHmGyBwUZXIcsGYMaUqGGSl4CFdx9Tozu-vQCn5bHIQbR7On7dZbU61vYvfrJr30t0iahSqhc64J46MnUO2JvQaGAYiOoCKKgmlkgnY0gmlwhAPnCzSHb3BzdGFja4OFQgCJc2VjcDI1NmsxoQINc4fSijfbNIiGhcgvwjsjxVFJHUstK9L1T8OTKUjgloN0Y3CCJAaDdWRwgiQG",
        "enr:-J24QG3ypT4xSu0gjb5PABCmVxZqBjVw9ca7pvsI8jl4KATYAnxBmfkaIuEqy9sKvDHKuNCsy57WwK9wTt2aQgcaDDyGAYiOoGAXgmlkgnY0gmlwhDbGmZaHb3BzdGFja4OFQgCJc2VjcDI1NmsxoQIeAK_--tcLEiu7HvoUlbV52MspE0uCocsx1f_rYvRenIN0Y3CCJAaDdWRwgiQG",

        // op-mainnet
        "enr:-Jy4QG7buuDG79C26iMjKkN2g9XMNTdGbt1YS-6HfH4m95veGWXZnUyCp38DF04xpP0mwZbwvrz4-VhaiGaED5jHS16GAZPf_rE7gmlkgnY0gmlwhEKHEzmHb3BzdGFja4IKAIlzZWNwMjU2azGhA3I_5vJvl9qRigYdq_vZFC4Kmomm5pN51RjUkWOZjkevg3RjcIIkBoN1ZHCCJAY",
        "enr:-Jy4QIvZvTWDR3Cbl33MQi8BfhCzdFHY6O4jLikkZrC2FskFSjGXplIuftBjDvT8OOdQcQMsZWTOIiPZaeJsex0wPrKGAZMMkqvfgmlkgnY0gmlwhDNRa3-Hb3BzdGFja4IKAIlzZWNwMjU2azGhA1EYgV9d0LWz_cev0L_OfVpG1jnxkBElMAwpKbDFj368g3RjcIJ2DoN1ZHCCdg4",
        "enr:-Jy4QJbH5o5igULthT5FDNkhSgVzWo2rHMhACFfYY3LsxrG0Ao2QnVpRpbXvVmVRM4plIYPaXbbDEtroIcfkX_DoV8SGAZQ7CDpHgmlkgnY0gmlwhHUUKFmHb3BzdGFja4IKAIlzZWNwMjU2azGhAm5KwnzPt3JtOYzEheMI64WYQrPBiF1EmQ48G-ce_KXag3RjcILt9IN1ZHCC7fQ",
        "enr:-Jy4QArQwW3R14zutEJrao_tS0qJG6A0Im0TeWf6hw8QWOPwS5xvrSj4yZcWQSLTonsYZXEkVGf-w5gw19CC9muuk-CGAZLsvteYgmlkgnY0gmlwhEEVFG-Hb3BzdGFja4IKAIlzZWNwMjU2azGhApCEjCMAzJWDFBDSZPlJotaHWfMhBDPhDiDPwGHGdqkQg3RjcIIkBoN1ZHCCv7Y",
        "enr:-Jy4QKUQuzZ-Z8Q2w0gyAaVa1nQi0_otmgnrg4zKDjOXj2d-Nv-XB7Wj-V6tKZhndFV-5UxzuUM6cnb9DKt47NZ0XbWGAZJ_Oqc9gmlkgnY0gmlwhFm7nGSHb3BzdGFja4IKAIlzZWNwMjU2azGhA73OCLtGYn3mupxhMEDISaiB0aXfqh9hC9OE_E14zx_Dg3RjcIJRpoN1ZHCCUaY",
        "enr:-Jy4QJxw_ycF8GBB6YxUdyMKOLuvaphJd8vLIsqkpTk_NOlgcO2wo-EQb5VUSDIpyjgXM9K8Vyzet7RZLSVYVTq9WHCGAY6ukVHEgmlkgnY0gmlwhA_M1USHb3BzdGFja4IKAIlzZWNwMjU2azGhAtr0g03NzP4vqoJjWqo2TBvl8geCLVZZ-g5plxRyt3Geg3RjcIIjK4N1ZHCCIys",
        "enr:-Jy4QEYwNQAtnlOqznwcG-N_cIVw6BHayOXBlaGdaVl6nDKoT-iHITk7JW5bpJwXyMA8Fsj7iGuAhqLuW1WR3mmwMNKGAZG8u1TNgmlkgnY0gmlwhEUKLTaHb3BzdGFja4IKAIlzZWNwMjU2azGhAs5BbWOy8BFVFVTmFuv6jaUwiK1QYqulICWmneWHYHmNg3RjcIJ6h4N1ZHCC6is",
        "enr:-Jy4QCpZMcw9ElQgPsUJL-HXrJCu3F4r3Zr1tfNpqk_3xKJEFsJPX_A20q5_Tq7N7jER2rZNtMLvDtx_Ypp9Rl49s6KGAZQ7OYg1gmlkgnY0gmlwhD_76H2Hb3BzdGFja4IKAIlzZWNwMjU2azGhAtKybUEYDh8vyGcHKvzecz1fAjv52gAL7aMgnrGHlsSeg3RjcIJQNIN1ZHCCUDQ",
    ];

    bootnodes
        .iter()
        .filter_map(|enr| Enr::from_str(enr).ok())
        .collect()
}
