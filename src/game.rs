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
    pub winners: Vec<AccountId>,
    pub winner_number: i32,
    pub total_prize: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    pub id: AccountId,
    pub number: i32,
    // pub participate_at: u64,
}

impl User {
    pub fn new(num: i32) -> Self {
        Self {
            id: env::signer_account_id(),
            number: num,
            // participate_at: env::block_timestamp(),
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
    pub winners: Vec<AccountId>,
    pub winner_number: i32,
    pub total_prize: u128,
}

impl GameMetaData {
    pub fn new() -> Self {
        Self {
            id: gen_game_id(),
            start_at: env::block_timestamp(),
            end_at: 0_u64,
            participants: Vec::new(),
            fee: JOIN_DEFAULT_FEE,
            winners: Vec::new(),
            winner_number: 100,
            total_prize: DEFAULT_PRIZE,
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn new_lottery_game(&mut self) -> GameId {
        if env::current_account_id() != env::signer_account_id() {
            assert!(false, "Only contract owner can start new game");
        }
        let game_id = self.lottery_games.get(&CURRENT_GAME_ID.to_string());

        assert!(!game_id.is_some(), "Lottery Game has been started!");
        return self.new_game();
    }

    fn new_game(&mut self) -> GameId {
        let mut game = GameMetaData::new();
        let game_id = game.clone().id;
        let previous_game_id = self.lottery_games.get(&PREVIOUS_GAME_ID.to_string());
        // .unwrap();
        if previous_game_id.is_some() {
            let previous_game = self.game_metadata.get(&previous_game_id.unwrap()).unwrap();
            let previous_winner_number = previous_game
                .winners
                .len()
                .to_string()
                .parse::<u128>()
                .unwrap();
            let previous_num_of_partis: u128 = previous_game
                .participants
                .len()
                .to_string()
                .parse::<u128>()
                .unwrap();
            let pre_percent = &PREVIOUS_REWARD_PERCENT.to_string().parse::<u128>().unwrap();
            let hundred: u128 = "100".parse::<u128>().unwrap();
            if previous_winner_number == 0 {
                game.total_prize = game.total_prize
                    + (previous_num_of_partis * previous_game.fee * pre_percent / hundred);
            }
        }
        self.lottery_games
            .insert(&CURRENT_GAME_ID.to_string(), &game_id);
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
            winners: res_game.winners,
            winner_number: res_game.winner_number,
            total_prize: res_game.total_prize.to_string(),
        };
    }

    pub fn get_user_ticket(&mut self, id: String, user_id: AccountId) -> i32 {
        let game = self.game_metadata.get(&id);
        assert!(game.is_some(), "Game does not exist!");
        let res_game = game.unwrap();
        for participant in res_game.participants.iter() {
            if participant.id == user_id {
                return participant.number;
            }
        }
        return 100;
    }

    pub fn get_current_game(&mut self) -> GameResponse {
        let game_id = self.lottery_games.get(&CURRENT_GAME_ID.to_string());
        // .unwrap();
        assert!(game_id.is_some(), "Game does not exist!");
        let game = self.game_metadata.get(&game_id.unwrap());
        assert!(game.is_some(), "Game does not exist!");

        let res_game = game.unwrap();

        return GameResponse {
            id: res_game.id,
            start_at: res_game.start_at,
            end_at: res_game.end_at,
            participants_number: res_game.participants.len(),
            fee: res_game.fee,
            winners: res_game.winners,
            winner_number: res_game.winner_number,
            total_prize: res_game.total_prize.to_string(),
        };
    }

    pub fn get_previous_game(&mut self) -> GameResponse {
        let game_id = self.lottery_games.get(&PREVIOUS_GAME_ID.to_string());
        // .unwrap();
        assert!(game_id.is_some(), "Game does not exist!");
        let game = self.game_metadata.get(&game_id.unwrap());
        assert!(game.is_some(), "Game does not exist!");

        let res_game = game.unwrap();

        return GameResponse {
            id: res_game.id,
            start_at: res_game.start_at,
            end_at: res_game.end_at,
            participants_number: res_game.participants.len(),
            fee: res_game.fee,
            winners: res_game.winners,
            winner_number: res_game.winner_number,
            total_prize: res_game.total_prize.to_string(),
        };
    }

    #[payable]
    pub fn buy_ticket(&mut self, num: i32) -> User {
        if env::current_account_id() == env::signer_account_id() {
            assert!(false, "Owner can not join the game!");
        }
        let current_game_id = self
            .lottery_games
            .get(&CURRENT_GAME_ID.to_string())
            .unwrap();
        let mut current_game = self.game_metadata.get(&current_game_id).unwrap();
        for participant in current_game.participants.iter() {
            if participant.id == env::signer_account_id() {
                assert!(false, "You have already bought this ticket!");
            }
        }
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
        if current_game.participants.len() == 100 {
            self.end_game();
        }
        return user;
    }

    #[payable]
    fn end_game(&mut self) -> Vec<AccountId> {
        let current_game_id = self
            .lottery_games
            .get(&CURRENT_GAME_ID.to_string())
            .unwrap();
        let mut current_game = self.game_metadata.get(&current_game_id).unwrap();
        let dec_number: u64 = env::block_timestamp() / 100;
        let number_str = (env::block_timestamp() - dec_number * 100).to_string();
        let winner_number: i32 = number_str.parse::<i32>().unwrap();
        let mut winners_vec: Vec<AccountId> = Vec::new();
        for participant in current_game.participants.iter() {
            if participant.number == winner_number {
                winners_vec.push(participant.clone().id);
            }
        }
        if winners_vec.clone().len() > 0 {
            let total_winner_amount: u128 = winners_vec.len().to_string().parse::<u128>().unwrap();
            let user_payment_amount: u128 = current_game.total_prize / total_winner_amount;
            for winner in winners_vec.clone() {
                Promise::new(winner).transfer(user_payment_amount);
            }
        }
        current_game.winners = winners_vec.clone();
        current_game.winner_number = winner_number;
        current_game.end_at = env::block_timestamp();
        self.game_metadata.insert(&current_game.id, &current_game);

        self.lottery_games
            .insert(&PREVIOUS_GAME_ID.to_string(), &current_game.id);
        // Create new Game
        self.new_game();

        // Send back 0.5 Near as execution fee for the user who call this method
        Promise::new(env::signer_account_id()).transfer(EXECUTION_CASH_BACK_FEE);

        return winners_vec.clone();
    }
}
