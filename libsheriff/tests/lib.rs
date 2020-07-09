#[cfg(test)]
mod tests {
    use libsheriff::{Entity, State};

    #[test]
    fn test_moving() {
        let mut state = State {
            current: "game".to_string(),
            stage_max_x: 10,
            stage_max_y: 5,
            entities: Default::default(),
        };


        let sheriff = Entity::new("sheriff","🤠", "s1", (0, 0));

        let s1 = sheriff.id.clone();

        state.entities.insert(sheriff.id.clone(), sheriff);

        let cow = Entity::new("cow", "🐄", "c1", (2, 2));

        state.entities.insert(cow.id.clone(), cow);

        // # Stage looks like
        // (Sheriff = S, Cow = C)
        // SS░░░░░░░░
        // ░░░░░░░░░░
        // ░░CC░░░░░░
        // ░░░░░░░░░░
        // ░░░░░░░░░░

        state.handle_move(&s1, 1, 1);

        let sheriff = state.entities.get(&s1).expect("cannot find sheriff");
        assert_eq!(sheriff.x, 1);
        assert_eq!(sheriff.y, 1);

        // # Stage looks like
        // (Sheriff = S, Cow = C)
        // ░░░░░░░░░░
        // ░SS░░░░░░░
        // ░░CC░░░░░░
        // ░░░░░░░░░░
        // ░░░░░░░░░░

        // now we should be blocked by the cow

        state.handle_move(&s1, 0, 1);
        state.handle_move(&s1, 0, 1);
        state.handle_move(&s1, 0, 1);
        state.handle_move(&s1, 0, 1);

        let sheriff = state.entities.get(&s1).expect("cannot find sheriff");

        assert_eq!(sheriff.x, 1);
        assert_eq!(sheriff.y, 1);

        // # Stage looks like
        // (Sheriff = S, Cow = C)
        // ░░░░░░░░░░
        // ░SS░░░░░░░
        // ░░CC░░░░░░
        // ░░░░░░░░░░
        // ░░░░░░░░░░

        state.handle_move(&s1, 1, 0);

        // # Stage looks like
        // (Sheriff = S, Cow = C)
        // ░░░░░░░░░░
        // ░░SS░░░░░░
        // ░░CC░░░░░░
        // ░░░░░░░░░░
        // ░░░░░░░░░░

        let sheriff = state.entities.get(&s1).expect("cannot find sheriff");

        assert_eq!(sheriff.x, 2);
        assert_eq!(sheriff.y, 1);

        // cannot escape stage
        state.handle_move(&s1, 1000, 0);
        let sheriff = state.entities.get(&s1).expect("cannot find sheriff");
        assert_eq!(sheriff.x, 8);
        assert_eq!(sheriff.y, 1);
        state.handle_move(&s1, 0, 1000);
        let sheriff = state.entities.get(&s1).expect("cannot find sheriff");
        assert_eq!(sheriff.x, 8);
        assert_eq!(sheriff.y, 4);
        state.handle_move(&s1, -1000, 0);
        let sheriff = state.entities.get(&s1).expect("cannot find sheriff");
        assert_eq!(sheriff.x, 0);
        assert_eq!(sheriff.y, 4);
        state.handle_move(&s1, 0, -1000);
        let sheriff = state.entities.get(&s1).expect("cannot find sheriff");
        assert_eq!(sheriff.x, 0);
        assert_eq!(sheriff.y, 0);

    }
}