pub mod fpscounter {
    use bevy::prelude::*;
    use std::time::SystemTime;
    pub struct FpsCounterPlugin;

    #[derive(Component)]
    pub struct FpsCounter {
        pub frame_nb: f32,
        pub now_time: SystemTime,
        pub avg_fps: f32,
        pub avg_nb: f32,
    }

    impl Plugin for FpsCounterPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, start_counter)
                .add_systems(Update, update_counter);
        }
    }

    fn start_counter(mut query: Query<&mut FpsCounter>) {
        let mut cnt = query.single_mut();
        cnt.frame_nb = 0.;
        cnt.now_time = SystemTime::UNIX_EPOCH;
        cnt.avg_nb = 30.;
    }

    fn update_counter(mut query: Query<&mut FpsCounter>) {
        let mut cnt = query.single_mut();
        cnt.frame_nb += 1.;
        if cnt.frame_nb == cnt.avg_nb {
            let now = SystemTime::now();
            let acc_time = now.duration_since(cnt.now_time).unwrap().as_secs_f32();
            let avg = cnt.avg_nb / acc_time;
            cnt.avg_fps = avg;
            cnt.frame_nb = 0.;
            cnt.now_time = SystemTime::now();
            println!("{}", avg);
        }
    }
}
