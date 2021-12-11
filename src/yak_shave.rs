use tracing::info;

#[tracing::instrument]
pub(crate) fn shave_all(number_of_yaks: i32) -> i32 {
    for yak_index in 0..number_of_yaks {
        info!(current_yak=yak_index+1, "Shaving in progress");
    }

    number_of_yaks
}