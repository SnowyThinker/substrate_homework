use crate::{mock::*, Error, Proofs};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		// 测试新建存证
		assert_ok!(PoeModule::create_claim(Origin::signed(1), vec![1]));

		//测试获取凭证
		assert_eq!(
			Proofs::<Test>::get(vec![1]),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		)
	});
}

/// 测试凭证已经存在
#[test]
fn create_clain_faled_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let _ = PoeModule::create_claim(Origin::signed(1), vec![1]);

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), vec![1]),
			Error::<Test>::ProofsAlreadyExists
		);
	});
}

/// 测试撤销凭证
#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let _ = PoeModule::create_claim(Origin::signed(1), vec![0, 1]);
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), vec![0, 1]));
	});
}

// 测试凭证不存在
#[test]
fn revoke_claim_failed_when_claim_is_not_exists() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), vec![0, 1]),
			Error::<Test>::ClaimNotExist
		);
	});
}

// 测试凭证owner不正确
#[test]
fn revoke_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let _ = PoeModule::create_claim(Origin::signed(1), vec![0, 1]);

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), vec![0, 1]),
			Error::<Test>::NotClaimOwner
		);
	});
}

// 测试转移凭证
#[test]
fn transfer_claim() {
	new_test_ext().execute_with(|| {
		//创建凭证
		let _ = PoeModule::create_claim(Origin::signed(1), vec![1]);
		//转移凭证
		let _ = PoeModule::transfer_claim(Origin::signed(1), vec![1], 2);
		
		//验证凭证所有者
		assert_eq!(
			Proofs::<Test>::get(vec![1]),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		)
	});
}