use metro_schedule::{NextArrivalRequest, NextArrivalResponse};

#[derive(Debug, Clone)]
pub(crate) struct MetroScheduleAPI {
    pub(crate) url: String,
}

impl Default for MetroScheduleAPI {
    fn default() -> Self {
        Self {
            url: "http://localhost:8000/next-arrival".into(),
        }
    }
}

impl MetroScheduleAPI {
    pub(crate) async fn next_arrival_request(
        &self,
        req: NextArrivalRequest,
    ) -> Result<NextArrivalResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let next_arrival: NextArrivalResponse = client
            .post(&self.url)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        Ok(next_arrival)
    }
}

//TODO: parse help messages
// pub(crate) fn help_schedule() -> &'static str {
//     "Next Arrival:\nGet the next arriving train on the STL Metro\nType East or West followed by a station name e.g. \"West fvh\"\nstation names:\n
//     lambert\n
//     lambert2\n
//     hanley\n
//     umsl north (umsl)\n
//     umsl south\n
//     rock road\n
//     wellston\n
//     delmar\n
//     shrewsbury\n
//     sunnen\n
//     maplewood\n
//     brentwood\n
//     richmond\n
//     clayton\n
//     forsyth\n
//     ucity\n
//     skinker\n
//     forest park\n
//     cwe (central west end)\n
//     cortex\n
//     grand\n
//     union\n
//     civic (civic center)\n
//     stadium\n
//     8th pine (8th and pine)\n
//     convention (convention center\n
//     lacledes (lacledes landing)\n
//     riverfront (east riverfront)\n
//     5th missouri (fifth missouri)\n
//     emerson\n
//     jjk (jackie joiner)\n
//     washington\n
//     fvh (fairview heights)\n
//     memorial hospital\n
//     swansea\n
//     belleville\n
//     college\n
//     shiloh (shiloh scott)"
// }
