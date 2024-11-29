#[cfg(test)]
#[test]
fn complete_game() {
    let mut test_cmds_vec = vec![" ".to_string()];

    test_cmds_vec.clear();
    //test_cmds_vec.push("save".to_string());

    test_cmds_vec.push("restore test_data/test_01_save.sst".to_string());

    test_cmds_vec.push("comment test_01 explore the galaxy".to_string());
    test_cmds_vec.push("jum 1 6 test".to_string());
    test_cmds_vec.push("jum 3 6 test".to_string());
    test_cmds_vec.push("jum 5 6 test".to_string());
    test_cmds_vec.push("jum 7 6 test".to_string());
    test_cmds_vec.push("jum 1 3 test".to_string());
    test_cmds_vec.push("jum 3 3 test".to_string());
    test_cmds_vec.push("jum 5 3 test".to_string());
    test_cmds_vec.push("jum 7 3 test".to_string());
    test_cmds_vec.push("jum 1 0 test".to_string());
    test_cmds_vec.push("jum 3 0 test".to_string());
    test_cmds_vec.push("jum 5 0 test".to_string());
    test_cmds_vec.push("jum 7 0 test".to_string());
    test_cmds_vec.push("lrs".to_string());

    //
    //
    //
    ///////////////////////////////////////////////////////////////////////////////////////////////////

    test_cmds_vec.push("comment row 7 starting".to_string());
    test_cmds_vec.push("stat".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 0 7 test3".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 2 7 test".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 3 7 test".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 4 7 test".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 5 7 test".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 7 7 test".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum sb".to_string());

    test_cmds_vec.push("comment row 6 test starting".to_string());

    test_cmds_vec.push("jum 6 6 test3".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 3 6 test".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum sb".to_string());

    test_cmds_vec.push("comment row 5 test starting".to_string());

    test_cmds_vec.push("jum 0 5 test3".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 1 5 test".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 2 5 test2".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 4 5 test".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 5 5 test".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 7 5 test".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum sb".to_string());

    test_cmds_vec.push("comment row 4 test starting".to_string());
    //test_cmds_vec.push("jum 0 4 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 1 4 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 2 4 test".to_string());
    test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 3 4 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 4 4 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 5 4 test".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 6 4 test2".to_string());
    test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 7 4 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum sb".to_string());

    test_cmds_vec.push("comment row 3 test starting".to_string());
    test_cmds_vec.push("jum 0 3 test".to_string());
    test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 1 3 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 2 3 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 3 3 test".to_string());
    test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 4 3 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 5 3 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 6 3 test2".to_string());
    //test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 7 3 test".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum sb".to_string());

    test_cmds_vec.push("comment row 2 test starting".to_string());
    test_cmds_vec.push("jum 0 2 test".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 1 2 test".to_string());
    test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 2 2 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 3 2 test".to_string());
    test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 4 2 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 5 2 test".to_string());
    test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 6 2 test2".to_string());
    //test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 7 2 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum sb".to_string());

    test_cmds_vec.push("comment row 1 test starting".to_string());
    //test_cmds_vec.push("jum 0 1 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 1 1 test".to_string());
    test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 2 1 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 3 1 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 4 1 test".to_string());
    test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 5 1 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 6 1 test2".to_string());
    //test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 7 1 test".to_string());
    test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum sb".to_string());

    //test_cmds_vec.push("comment ===>".to_string());

    test_cmds_vec.push("comment row 0 test starting".to_string());
    test_cmds_vec.push("jum 0 0 test".to_string());
    test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 1 0 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 2 0 test".to_string());
    test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 3 0 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 4 0 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 5 0 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum 6 0 test2".to_string());
    test_cmds_vec.push("tor".to_string());
    //test_cmds_vec.push("jum 7 0 test".to_string());
    //test_cmds_vec.push("tor".to_string());
    test_cmds_vec.push("jum sb".to_string());

    test_cmds_vec.push("comment ".to_string());

    test_cmds_vec.push("comment phaser starting".to_string());

    test_cmds_vec.push("comment ".to_string());

    //
    /*    test_cmds_vec.push("comment row 4 test starting".to_string());
    //

        //test_cmds_vec.push("jum 0 4 test".to_string());
        //test_cmds_vec.push("pha".to_string());
        //test_cmds_vec.push("jum 1 4 test".to_string());
        //test_cmds_vec.push("pha".to_string());
        test_cmds_vec.push("jum 2 4 test".to_string());
        test_cmds_vec.push("pha".to_string());
        //test_cmds_vec.push("jum 3 4 test".to_string());
        //test_cmds_vec.push("pha".to_string());
        //test_cmds_vec.push("jum 4 4 test".to_string());
        //test_cmds_vec.push("pha".to_string());
        test_cmds_vec.push("jum 5 4 test".to_string());
        test_cmds_vec.push("pha".to_string());
        test_cmds_vec.push("jum 6 4 test2".to_string());
        test_cmds_vec.push("pha".to_string());
        //test_cmds_vec.push("jum 7 4 test".to_string());
        //test_cmds_vec.push("pha".to_string());
        test_cmds_vec.push("jum sb".to_string());

        test_cmds_vec.push("comment phaser row 3 test starting".to_string());
        test_cmds_vec.push("jum 0 3 test".to_string());
        test_cmds_vec.push("pha".to_string());
        test_cmds_vec.push("jum 1 3 test".to_string());
        test_cmds_vec.push("pha".to_string());
        test_cmds_vec.push("jum 2 3 test".to_string());
        test_cmds_vec.push("pha".to_string());
        test_cmds_vec.push("jum 3 3 test".to_string());
        test_cmds_vec.push("pha".to_string());
        test_cmds_vec.push("jum 4 3 test".to_string());
        test_cmds_vec.push("pha".to_string());
        test_cmds_vec.push("jum 5 3 test".to_string());
        test_cmds_vec.push("pha".to_string());
        test_cmds_vec.push("jum 6 3 test2".to_string());
        test_cmds_vec.push("pha".to_string());
        test_cmds_vec.push("jum 7 3 test".to_string());
        test_cmds_vec.push("pha".to_string());
        test_cmds_vec.push("jum sb".to_string());

    */
    //
    //
    //
    //

    test_cmds_vec.push("comment phaser row 7 test starting".to_string());
    //test_cmds_vec.push("jum 0 7 test".to_string());
    //test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 1 7 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 2 7 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 3 7 test".to_string());
    test_cmds_vec.push("pha".to_string());
    //test_cmds_vec.push("jum 4 7 test".to_string());
    //test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 5 7 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 6 7 test2".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 7 7 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum sb".to_string());

    test_cmds_vec.push("comment phaser row 6 test starting".to_string());
    test_cmds_vec.push("jum 0 6 test".to_string());
    test_cmds_vec.push("pha".to_string());
    //test_cmds_vec.push("jum 1 6 test".to_string());
    //test_cmds_vec.push("pha".to_string());
    //test_cmds_vec.push("jum 2 6 test".to_string());
    //test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 3 6 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 4 6 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 5 6 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 6 6 test2".to_string());
    test_cmds_vec.push("pha".to_string());
    //test_cmds_vec.push("jum 7 6 test".to_string());
    //test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum sb".to_string());

    test_cmds_vec.push("comment phaser row 5 test starting".to_string());
    test_cmds_vec.push("jum 0 5 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 1 5 test".to_string());
    test_cmds_vec.push("pha".to_string());
    //test_cmds_vec.push("jum 2 5 test".to_string());
    //test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 3 5 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 4 5 test".to_string());
    test_cmds_vec.push("pha".to_string());
    //test_cmds_vec.push("jum 5 5 test".to_string());
    //test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 6 5 test2".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 7 5 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum sb".to_string());

    test_cmds_vec.push("comment phaser row 4 test starting".to_string());
    test_cmds_vec.push("jum 0 4 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 1 4 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 2 4 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 3 4 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 4 4 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 5 4 test".to_string());
    test_cmds_vec.push("pha".to_string());
    //test_cmds_vec.push("jum 6 4 test2".to_string());
    //test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 7 4 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum sb".to_string());

    test_cmds_vec.push("comment phaser row 3 test starting".to_string());
    test_cmds_vec.push("jum 0 3 test2".to_string());
    test_cmds_vec.push("pha".to_string());
    //test_cmds_vec.push("jum 1 3 test".to_string());
    //test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 2 3 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 3 3 test".to_string());
    test_cmds_vec.push("pha".to_string());
    //test_cmds_vec.push("jum 4 3 test".to_string());
    //test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 5 3 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 6 3 test2".to_string());
    test_cmds_vec.push("pha".to_string());
    //test_cmds_vec.push("jum 7 3 test".to_string());
    //test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum sb".to_string());

    test_cmds_vec.push("comment phaser row 2 test starting".to_string());
    test_cmds_vec.push("jum 0 2 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 1 2 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 2 2 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 3 2 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 4 2 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 5 2 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 6 2 test2".to_string());
    test_cmds_vec.push("pha".to_string());
    //test_cmds_vec.push("jum 7 2 test".to_string());
    //test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum sb".to_string());

    test_cmds_vec.push("comment phaser row 1 test starting".to_string());
    //test_cmds_vec.push("jum 0 1 test".to_string());
    //test_cmds_vec.push("pha".to_string());
    //test_cmds_vec.push("jum 1 1 test".to_string());
    //test_cmds_vec.push("pha".to_string());
    //test_cmds_vec.push("jum 2 1 test".to_string());
    //test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 3 1 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 4 1 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 5 1 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 6 1 test2".to_string());
    test_cmds_vec.push("phar".to_string());
    test_cmds_vec.push("jum 7 1 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum sb".to_string());

    test_cmds_vec.push("comment phaser row 0 test starting".to_string());
    test_cmds_vec.push("jum 0 0 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 1 0 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 2 0 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 3 0 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 4 0 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 5 0 test".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 6 0 test2".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum 7 0 test2".to_string());
    test_cmds_vec.push("pha".to_string());
    test_cmds_vec.push("jum sb".to_string());

    //Refuel
    test_cmds_vec.push("stat".to_string());
    test_cmds_vec.push("lrs".to_string());
    test_cmds_vec.push("qui".to_string());
    
    //test_cmds_vec.push("stat".to_string());





    crate::write_toml(test_cmds_vec.clone());
   println!("{}", crate::read_toml());
    //crate::read_toml();
    //super::ui::cmd_proc::command_processor(&test_cmds_vec);
    //let result = add(2, 2);
    //assert_eq!(result, 4);
}
//}
