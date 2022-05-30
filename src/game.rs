pub use crate::constants::*;
use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Game {
    pub owner_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct GameResponse {
    pub id: String,
    pub start_at: u64,
    pub end_at: u64,
    pub participants_number: usize,
    pub fee: Balance,
    pub winner: User,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    pub id: AccountId,
    pub number: i32,
    pub participate_at: u64,
}

impl User {
    pub fn new(num: i32) -> Self {
        Self {
            id: env::signer_account_id(),
            number: num,
            participate_at: env::block_timestamp(),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct GameMetaData {
    pub id: String,
    pub start_at: u64,
    pub end_at: u64,
    pub participants: Vec<User>,
    pub fee: Balance,
    pub winner: User,
}

impl GameMetaData {
    pub fn new() -> Self {
        Self {
            id: gen_game_id(),
            start_at: env::block_timestamp(),
            end_at: 0_u64,
            participants: Vec::new(),
            fee: JOIN_DEFAULT_FEE,
            winner: User::new(101),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn new_game(&mut self) -> GameId {
        let owner = env::current_account_id();
        if owner != env::signer_account_id() {
            assert!(
                false,
                "Sorry, you have no permission to access this service"
            );
        }
        let game = GameMetaData::new();
        let game_id = game.clone().id;
        self.lottery_games.insert(&CURRENT_GAME_ID.to_string(), &game_id);
        self.game_metadata.insert(&game_id, &game);
        return game_id;
    }

    pub fn get_game(&mut self, id: String) -> GameResponse {
        let game = self.game_metadata.get(&id);
        assert!(game.is_some(), "Game does not exist!");

        let res_game = game.unwrap();

        return GameResponse {
            id: res_game.id,
            start_at: res_game.start_at,
            end_at: res_game.end_at,
            participants_number: res_game.participants.len(),
            fee: res_game.fee,
            winner: res_game.winner,
        };
    }

    pub fn get_current_game(&mut self) -> GameResponse {
        let game_id = self.lottery_games.get(&CURRENT_GAME_ID.to_string()).unwrap();
        let game = self.game_metadata.get(&game_id);
        assert!(game.is_some(), "Game does not exist!");

        let res_game = game.unwrap();

        return GameResponse {
            id: res_game.id,
            start_at: res_game.start_at,
            end_at: res_game.end_at,
            participants_number: res_game.participants.len(),
            fee: res_game.fee,
            winner: res_game.winner,
        };
    }

    pub fn get_previous_game(&mut self) -> GameResponse {
        let game_id = self.lottery_games.get(&PREVIOUS_GAME_ID.to_string()).unwrap();
        let game = self.game_metadata.get(&game_id);
        assert!(game.is_some(), "Game does not exist!");

        let res_game = game.unwrap();

        return GameResponse {
            id: res_game.id,
            start_at: res_game.start_at,
            end_at: res_game.end_at,
            participants_number: res_game.participants.len(),
            fee: res_game.fee,
            winner: res_game.winner,
        };
    }

    pub fn buy_ticket(&mut self, num: i32) -> User {
        let current_game_id = self.lottery_games.get(&CURRENT_GAME_ID.to_string()).unwrap();
        let mut current_game = self.game_metadata.get(&current_game_id).unwrap();
        let dep = env::attached_deposit();
        if dep < current_game.fee {
            assert!(false, "Not Enough Fee!");
        }
        if num >= 100 || num < 0 {
            assert!(false, "Invalid Number!");
        }
        let user = User::new(num);
        current_game.participants.push(user.clone());
        self.game_metadata.insert(&current_game_id, &current_game);
        return user;
    }

    #[payable]
    pub fn end_game(&mut self) -> User {
        let current_game_id = self.lottery_games.get(&CURRENT_GAME_ID.to_string()).unwrap();
        let mut current_game = self.game_metadata.get(&current_game_id).unwrap();
        let dec_number: u64 = env::block_timestamp() / 100;
        let number_str = (env::block_timestamp() - dec_number * 100).to_string();
        let winner_number:i32 = number_str.parse::<i32>().unwrap();
        let mut cr_winner: &User = &User { id: self.owner_id.clone(), number: 100, participate_at: env::block_timestamp() };
        for participant in current_game.participants.iter() {
            if participant.number == winner_number && participant.participate_at < cr_winner.participate_at {
                cr_winner = participant;
            }
        }

        let res = cr_winner.clone();
        let winner = cr_winner.clone();
        if winner.id != self.owner_id.clone() {
            let u128_num_of_partis: u128 = current_game.participants.len().to_string().parse::<u128>().unwrap();
            let percent = &REWARD_PERCENT.to_string().parse::<u128>().unwrap();
            let hundred: u128 = "100".parse::<u128>().unwrap();
            let reward_amount: u128 = (current_game.fee * u128_num_of_partis) * percent / hundred;
            Promise::new(winner.id).transfer(reward_amount);
        }
        current_game.winner = cr_winner.clone();
        self.game_metadata.insert(&current_game.id, &current_game);
        self.lottery_games.insert(&CURRENT_GAME_ID.to_string(), &String::from(""));
        self.lottery_games.insert(&PREVIOUS_GAME_ID.to_string(), &current_game.id);
        return res;
    }

}
