use crate::{Error, mock::*, Event};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_create() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let name = *b"abcd1234";

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), name));

        // let _res = Balances::set_balance(RuntimeOrigin::signed(account_id), account_id, 1_000_000_000, 1_000_000_000);

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
        assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

        crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
        assert_noop!(
            KittiesModule::create(RuntimeOrigin::signed(account_id), name),
            Error::<Test>::InvalidKittyId,
        );
    });
}

#[test]
fn it_works_for_breed() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let name = *b"abcd1234";

        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id, name),
            Error::<Test>::SameKittyId
        );

        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1, name),
            Error::<Test>::InvalidKittyId
        );

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), name));
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), name));

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

        assert_ok!(KittiesModule::breed(
            RuntimeOrigin::signed(account_id), 
            kitty_id, 
            kitty_id + 1,
            name
        ));

        let breed_kitty_id = 2;
        assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
        assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
        assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));

        assert_eq!(
            KittiesModule::kitty_parents(breed_kitty_id),
            Some((kitty_id, kitty_id + 1))
        );

    });
}

#[test]
fn it_works_for_transfer() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let recipient = 2;
        let name = *b"abcd1234";

        // let _res = Balances::set_balance(RuntimeOrigin::signed(account_id), account_id, 1_000_000_000, 1_000_000_000);

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), name));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

        // 从recipient 转账到account 会报错
        assert_noop!(KittiesModule::transfer(
                RuntimeOrigin::signed(recipient), 
                account_id, 
                kitty_id
            ),
            Error::<Test>::NotOwner
        );

        // 从account 转账到 recipient 正常
        assert_ok!(KittiesModule::transfer(
            RuntimeOrigin::signed(account_id), 
            recipient, 
            kitty_id
        ));

        // 转账后判断 owner是不是recipient 
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));

        // 再从recipient 转账到 account
        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient), account_id, kitty_id));

        // 最后判断kiity是符是最初的owner
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));    

        // 接收最近转账的事件，并比较是符相同
        //System::assert_last_event(Event::KittyTransferred { who: recipient, recipient: account_id, kitty_id: kitty_id }.into());    
    });
}

#[test]
fn it_works_for_buy() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let name = *b"abcd1234";

        KittiesModule::create(RuntimeOrigin::signed(account_id), name).unwrap();

        // assert_noop!(
        //     KittiesModule::buy(
        //         RuntimeOrigin::signed(account_id), 
        //         2
        //     ),
        //     Error::<Test>::AlreadyOwned
        // );

        // assert_noop!(
        //     KittiesModule::buy(
        //         RuntimeOrigin::signed(account_id), 
        //         2
        //     ),
        //     Error::<Test>::NotOnSale
        // );

        let _ = KittiesModule::buy(RuntimeOrigin::signed(account_id), kitty_id);

        // 转账后判断 owner是不是recipient 
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

        // System::assert_last_event(Event::KittyBought {who: account_id, kitty_id: kitty_id}.into());
    });
}

#[test]
pub fn it_work_for_sale() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let name = *b"abcd1234";

        KittiesModule::create(RuntimeOrigin::signed(account_id), name).unwrap();
        KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id).unwrap();

        // KittiesModule::buy(RuntimeOrigin::signed(2), kitty_id).unwrap();

        //System::assert_last_event(Event::KittyOnSale { who: account_id, kitty_id: kitty_id }.into());
    });
}