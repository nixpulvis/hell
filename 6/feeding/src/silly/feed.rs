use interact::*;
use silly::*;

impl Choose<FeedObservation, FeedChoice> for Silly {
    fn choose(&mut self, observation: &FeedObservation) -> Result<Option<FeedChoice>, ()> {
        let current_player_idx = observation.current_player_index();
        let all_choices = observation.choices();
        let considered_choices = all_choices.into_iter().filter(|&c| {
            match c {
                FeedChoice::Attack(_, target_idx, _) => {
                    current_player_idx != target_idx
                }
                _ => true
            }
        }).collect::<Vec<_>>();
        Self::best_feed_choice(observation, considered_choices)
    }
}

impl Silly {
    /// Given a vector of choices, return the lowest ordered one (best)
    /// based on the implementation of `RankedFeedChoice`.
    fn best_feed_choice(observation: &FeedObservation,
                        choices: Vec<FeedChoice>) -> Result<Option<FeedChoice>, ()>
    {
        if choices.len() > 0 {
            let mut ranked: Vec<RankedFeedChoice> =
                choices.into_iter().map(|c| RankedFeedChoice(c, &observation)).collect();
            ranked.sort();
            Ok(Some(ranked.remove(0).into()))
        } else {
            Ok(None)
        }
    }
}
