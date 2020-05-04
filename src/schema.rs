table! {
    Bet (ID) {
        ID -> Integer,
        Who -> Nullable<Integer>,
        Value -> Nullable<Double>,
        on_run -> Nullable<Integer>,
        on_winner -> Nullable<Integer>,
        win_rate -> Nullable<Double>,
    }
}

table! {
    CompetList (Run_compet, HorseID) {
        Run_compet -> Integer,
        HorseID -> Integer,
    }
}

table! {
    Horses (ID) {
        ID -> Integer,
        Name -> Nullable<Varchar>,
        Owner -> Nullable<Integer>,
        Age -> Nullable<Integer>,
        WinRate -> Nullable<Double>,
        RunsDone -> Integer,
    }
}

table! {
    Owners (ID) {
        ID -> Integer,
        Name -> Nullable<Varchar>,
        Surname -> Nullable<Varchar>,
        Age -> Nullable<Integer>,
        UUID -> Nullable<Integer>,
    }
}

table! {
    Payments (ID) {
        ID -> Integer,
        Other_side -> Nullable<Integer>,
        Value -> Nullable<Integer>,
        Outcoming -> Nullable<Bool>,
        State -> Nullable<Integer>,
    }
}

table! {
    Run (ID) {
        ID -> Integer,
        DateOf -> Nullable<Date>,
        Place -> Nullable<Varchar>,
        Winner -> Nullable<Integer>,
        CompetitorsList -> Nullable<Integer>,
    }
}

table! {
    UserData (ID) {
        ID -> Integer,
        Login -> Nullable<Varchar>,
        Passwh -> Nullable<Binary>,
        UserType -> Nullable<Integer>,
        Credits -> Nullable<Integer>,
        Balance -> Double,
        AssocInf -> Nullable<Varchar>,
        PublicProfile -> Integer,
    }
}

joinable!(Bet -> Run (on_run));
joinable!(Bet -> UserData (Who));
joinable!(CompetList -> Horses (HorseID));
joinable!(Horses -> Owners (Owner));
joinable!(Run -> Horses (Winner));

allow_tables_to_appear_in_same_query!(
    Bet,
    CompetList,
    Horses,
    Owners,
    Payments,
    Run,
    UserData,
);
