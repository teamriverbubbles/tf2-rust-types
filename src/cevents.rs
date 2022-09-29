use libc::{c_int, c_char, c_float, c_long, c_short};

#[repr(C)]
pub struct achievement_earned_local {
    pub achievement: c_short,

}

#[repr(C)]
pub struct achievement_earned {
    pub player: c_short,
    pub achievement: c_short,
}

#[repr(C)]
pub struct air_dash {
    pub player: c_short,
}

#[repr(C)]
pub struct arena_match_maxstreak {
    pub team: c_short,
    pub streak: c_short,
}

#[repr(C)]
pub struct arena_player_notification {
    pub player: c_short,
    pub message: c_char,
}

#[repr(C)]
pub struct arena_win_panel {
    pub paneltyle: c_short,
    pub winning_team: c_short,
    pub winreason: c_char,
    pub cappers: c_char,
    pub flagcaplimit: c_short,
    pub bluecore: c_short,
    pub redcore: c_short,
    pub bluecore_prev: c_short,
    pub redcore_prev: c_short,
    pub round_complete: c_short,
    pub player_1: c_short,
    pub player_1_damage: c_short,
    pub player_1_healing: c_short,
    pub player_1_lifetime: c_short,
    pub player_1_kills: c_short,
    pub player_2: c_short,
    pub player_2_damage: c_short,
    pub player_2_healing: c_short,
    pub player_2_lifetime: c_short,
    pub player_2_kills: c_short,
    pub player_3: c_short,
    pub player_3_damage: c_short,
    pub player_3_healing: c_short,
    pub player_3_lifetime: c_short,
    pub player_3_kills: c_short,
    pub player_4: c_short,
    pub player_4_damage: c_short,
    pub player_4_healing: c_short,
    pub player_4_lifetime: c_short,
    pub player_4_kills: c_short,
    pub player_5: c_short,
    pub player_5_damage: c_short,
    pub player_5_healing: c_short,
    pub player_5_lifetime: c_short,
    pub player_5_kills: c_short,
    pub player_6: c_short,
    pub player_6_damage: c_short,
    pub player_6_healing: c_short,
    pub player_6_lifetime: c_short,
    pub player_6_kills: c_short,
}

#[repr(C)]
pub struct arrow_impact {
    pub attachedEntity: c_short,
    pub shooter: c_short,
    pub boneIndexAttached: c_short,
    pub bonePositionX: c_float,
    pub bonePositionY: c_float,
    pub bonePositionZ: c_float,
    pub boneAnglesX: c_float,
    pub boneAnglesY: c_float,
    pub boneAnglesZ: c_float,
    pub projectileType: c_short,
    pub isCrit: bool,
}

#[repr(C)]
pub struct building_healed {
    pub building: c_short,
    pub healer: c_short,
    pub amount: c_short,
}

#[repr(C)]
pub struct building_info_changed {
    pub building_type: c_short,
    pub object_mode	: c_short,
    pub remove: c_short,
}

#[repr(C)]
pub struct capper_killed {
    pub blocker: c_short,
    pub victim: c_short,

}

#[repr(C)]
pub struct christmas_gift_grab {
    pub userid: c_short,
}

#[repr(C)]
pub struct cl_drawline {
    pub player: c_short,
    pub panel: c_short,
    pub line: c_short,
    pub x: c_float,
    pub y: c_float,
}

#[repr(C)]
pub struct competitivetats_update {
    pub index: c_short,
    pub kills_rank: c_short,
    pub score_rank: c_short,
    pub damage_rank: c_short,
    pub healing_rank: c_short,
    pub support_rank: c_short,
}

#[repr(C)]
pub struct conga_kill {
    pub index: c_short,
}

#[repr(C)]
pub struct controlpoint_endtouch {
    pub player: c_short,
    pub area: c_short,
}

#[repr(C)]
pub struct controlpoint_fake_capture_mult {
    pub player: c_short,
    pub int_data: c_short,
}

#[repr(C)]
pub struct controlpoint_pulse_element {
    pub player: c_short,
}

#[repr(C)]
pub struct controlpointtarttouch {
    pub player: c_short,
    pub area: c_short,
}

#[repr(C)]
pub struct controlpoint_timer_updated_s {
    pub entindex: c_short,
    pub time: c_float,
}

#[repr(C)]
pub struct controlpoint_unlock_updated_s {
    pub index: c_short,
    pub time: c_float,
}

#[repr(C)]
pub struct controlpoint_updatecapping_s {
    pub index: c_short,
}

#[repr(C)]
pub struct controlpoint_updateimages_s {
    pub index: c_short,
}

#[repr(C)]
pub struct controlpoint_updateowner_s {
    pub index: c_short,
}

#[repr(C)]
pub struct cross_spectral_bridge_s {
    pub player: c_short,
}

#[repr(C)]
pub struct crossbow_heal_s {
    pub healer: c_short,
    pub target: c_short,
    pub amount: c_short,
}

#[repr(C)]
pub struct ctf_flag_captured_s {
    pub capping_team: c_short,
    pub capping_team_score	: c_short,
}

#[repr(C)]
pub struct damage_mitigated_s {
    pub mitigator: c_short,
    pub damaged: c_short,
    pub amount: c_short,
    pub itemdefindex: c_short,
}

#[repr(C)]
pub struct damage_prevented_s {
    pub preventor: c_short,
    pub victim: c_short,
    pub amount: c_short,
    pub condition: c_short,
}

#[repr(C)]
pub struct damage_resisted_s {
    pub entindex: c_short,
}

#[repr(C)]
pub struct demoman_det_stickies_s {
    pub player: c_short,
}

#[repr(C)]
pub struct deploy_buff_banner_s {
    pub buff_type: c_short,
    pub buff_owner: c_short,
}

#[repr(C)]
pub struct doomsday_rocket_open_s {
    pub team: c_short,
}

#[repr(C)]
pub struct duel_status_s {
    pub killer: c_short,
    pub score_type: c_short,
    pub initiator: c_short,
    pub target: c_short,
    pub initiator_score: c_short,
    pub target_score: c_short,
}

#[repr(C)]
pub struct environmental_death_s {
    pub killer: c_short,
    pub victim: c_short,
}

#[repr(C)]
pub struct escaped_hell_s {
    pub player: c_short,
}

#[repr(C)]
pub struct escaped_loot_island_s {
    pub player: c_short,
}

#[repr(C)]
pub struct escort_progress_s {
    pub team: c_short,
    pub progress: c_float,
    pub reset: bool,
}

#[repr(C)]
pub struct escort_recede_s {
    pub team: c_short,
    pub recedetime: c_float,
}

#[repr(C)]
pub struct escort_speed_s {
    pub team: c_short,
    pub speed: c_char,
    pub players: c_short,
}

#[repr(C)]
pub struct eyeball_boss_escape_imminent_s {
    pub level: c_short,
    pub time_remaining: c_char,
}

#[repr(C)]
pub struct eyeball_boss_escaped_s {
    pub level: c_short,

}

#[repr(C)]
pub struct eyeball_boss_killed_s {
    pub level: c_long,
}

#[repr(C)]
pub struct eyeball_boss_killer_s {
    pub level: c_short,
    pub player_entindex: c_long,
}

#[repr(C)]
pub struct eyeball_boss_stunned_s {
    pub level: c_short,
    pub player_entindex: c_long,
}

#[repr(C)]
pub struct fish_notice_s {
    pub userid: c_short,
    pub victim_entindex: c_long,
    pub inflictor_entindex: c_long,
    pub attacker: c_short,
    pub weapon: c_char,
    pub weaponid: c_short,
    pub damagebits: c_long,
    pub customkill: c_short,
    pub assister: c_short,
    pub weapon_logclassname: c_char,
    pub stun_flags: c_short,
    pub death_flags: c_short,
    pub silent_kill: bool,
    pub assister_fallback: c_char,
}

#[repr(C)]
pub struct flagstatus_update_s {
    pub userid: c_short,
    pub entindex: c_long,
}

#[repr(C)]
pub struct gas_doused_player_ignited_s {
    pub igniter: c_short,
    pub douser: c_short,
    pub victim: c_short,
}

#[repr(C)]
pub struct halloween_boss_killed_s {
    pub boss: c_short,
    pub killer: c_short,
}

#[repr(C)]
pub struct halloween_duck_collected_s {
    pub collector: c_short,
}

#[repr(C)]
pub struct halloween_pumpkin_grab_s {
    pub userid: c_short,
}

#[repr(C)]
pub struct halloween_skeleton_killed_s {
    pub player: c_short,
}

#[repr(C)]
pub struct halloween_soul_collected_s {
    pub intended_target: c_short,
    pub collecting_player: c_short,
    pub soul_count: c_short,
}

#[repr(C)]
pub struct hide_annotation_s {
    pub id: c_long,
}

#[repr(C)]
pub struct intro_finish_s {
    pub player: c_short,
}

#[repr(C)]
pub struct intro_nextcamera_s {
    pub player: c_short,
}

#[repr(C)]
pub struct item_found_s {
    pub player: c_short,
    pub quality: c_short,
    pub method: c_short,
    pub itemdef: c_long,
    pub isstrange: c_short,
    pub isunusual: c_short,
    pub wear: c_float,
}

#[repr(C)]
pub struct item_pickup_s {
    pub userid: c_short,
    pub item: c_char,
}

#[repr(C)]
pub struct kill_in_hell_s {
    pub killer: c_short,
    pub victim: c_short,
}

#[repr(C)]
pub struct kill_refills_meter_s {
    pub index: c_short,
}

#[repr(C)]
pub struct killed_capping_player_s {
    pub cp: c_short,
    pub killer: c_short,
    pub victim: c_short,
    pub assister: c_short,
}

#[repr(C)]
pub struct landed_s {
    pub player: c_short,
}

#[repr(C)]
pub struct localplayer_changedisguise_s {
    pub disguised: bool,
}

#[repr(C)]
pub struct localplayer_healed_s {
    pub amount: c_short,
}

#[repr(C)]
pub struct localplayer_score_changed_s {
    pub score: c_short,
}

#[repr(C)]
pub struct medic_death_s {
    pub userid: c_short,
    pub attacker: c_short,
    pub healing: c_short,
    pub charged: bool,
}

#[repr(C)]
pub struct medigun_shield_blocked_damage_s {
    pub userid: c_short,
    pub damage: c_float,
}

#[repr(C)]
pub struct merasmus_escape_warning_s {
    pub level: c_short,
    pub time_remaining: c_char,
}

#[repr(C)]
pub struct merasmus_escaped_s {
    pub level: c_short,
}

#[repr(C)]
pub struct merasmus_prop_found_s {
    pub player: c_short,
}

#[repr(C)]
pub struct merasmus_stunned_s {
    pub player: c_short,
}

#[repr(C)]
pub struct minigame_win_s {
    pub team: c_short,
    pub types: c_short,
}

#[repr(C)]
pub struct minigame_won_s {
    pub player: c_short,
    pub game: c_short,
}

#[repr(C)]
pub struct mvm_adv_wave_complete_no_gates_s {
    pub index: c_short,
}

#[repr(C)]
pub struct mvm_begin_wave_s {
    pub wave_index: c_short,
    pub max_waves: c_short,
    pub advanced: c_short,
}

#[repr(C)]
pub struct mvm_bomb_carrier_killed_s {
    pub level: c_short,
}

#[repr(C)]
pub struct mvm_bomb_deploy_reset_by_player_s {
    pub player: c_short,
}

#[repr(C)]
pub struct mvm_bomb_reset_by_player_s {
    pub player: c_short,
}

#[repr(C)]
pub struct mvm_kill_robot_delivering_bomb_s {
    pub player: c_short,
}

#[repr(C)]
pub struct mvm_medic_powerup_shared_s {
    pub player: c_short,
}

#[repr(C)]
pub struct mvm_mission_complete_s {
    pub mission: c_char,
}

#[repr(C)]
pub struct mvm_mission_update_s {
    pub class	: c_short,
    pub count: c_short,
}

#[repr(C)]
pub struct mvm_pickup_currency_s {
    pub player: c_short,
    pub currency: c_short,
}

#[repr(C)]
pub struct mvm_quick_sentry_upgrade_s {
    pub player: c_short,
}

#[repr(C)]
pub struct mvm_scout_marked_for_death_s {
    pub player: c_short,
}

#[repr(C)]
pub struct mvm_sentrybuster_detonate_s {
    pub player: c_short,
    pub det_x: c_float,
    pub det_y: c_float,
    pub det_z: c_float,
}

#[repr(C)]
pub struct mvm_sentrybuster_killed_s {
    pub sentry_buster: c_short,
}

#[repr(C)]
pub struct mvm_sniper_headshot_currency_s {
    pub userid: c_short,
    pub currency: c_short,
}

#[repr(C)]
pub struct mvm_wave_complete_s {
    pub advanced: bool,
}

#[repr(C)]
pub struct nav_blocked_s {
    pub area: c_long,
    pub blocked: bool,
}

#[repr(C)]
pub struct num_cappers_changed_s {
    pub index: c_short,
    pub count: c_int,
}

#[repr(C)]
pub struct object_deflected_s {
    pub userid: c_short,
    pub ownerid: c_short,
    pub weaponid: c_short,
    pub object_entindex: c_long,
}

#[repr(C)]
pub struct object_destroyed_s {
    pub userid: c_short,
    pub attacker: c_short,
    pub assister: c_short,
    pub weapon: c_char,
    pub weaponid: c_short,
    pub objecttype: c_short,
    pub index: c_short,
    pub was_building: bool,
}

#[repr(C)]
pub struct object_detonated_s {
    pub userid: c_short,
    pub objecttype: c_short,
    pub index: c_short,
}

#[repr(C)]
pub struct object_removed_s {
    pub userid: c_short,
    pub objecttype: c_short,
    pub index: c_short,
}

#[repr(C)]
pub struct parachute_deploy_s {
    pub index: c_short,
}

#[repr(C)]
pub struct parachute_holster_s {
    pub index: c_short,
}

#[repr(C)]
pub struct party_chat_s {
    pub steamid: c_char,
    pub text: c_char,
    pub types: c_short,
}

#[repr(C)]
pub struct party_member_join_s {
    pub steamid: c_char,
}

#[repr(C)]
pub struct party_member_leave_s {
    pub steamid: c_char,
}

#[repr(C)]
pub struct pass_ball_blocked_s {
    pub owner: c_short,
    pub blocker: c_short,
}

#[repr(C)]
pub struct pass_ball_stolen_s {
    pub victim: c_short,
    pub attacker: c_short,
}

#[repr(C)]
pub struct pass_free_s {
    pub owner: c_short,
    pub attacker: c_short,
}

#[repr(C)]
pub struct pass_get_s {
    pub owner: c_short,
}

#[repr(C)]
pub struct pass_pass_caught_s {
    pub passer: c_short,
    pub catcher: c_short,
    pub dist: c_float,
    pub duration: c_float,
}

#[repr(C)]
pub struct pass_score_s {
    pub scorer: c_short,
    pub assister: c_short,
    pub points: c_short,
}

#[repr(C)]
pub struct path_track_passed_s {
    pub index: c_short,
}

#[repr(C)]
pub struct payload_pushed_s {
    pub pusher: c_short,
    pub distance: c_char,
}

#[repr(C)]
pub struct player_abandoned_match_s {
    pub game_over: bool,
}

#[repr(C)]
pub struct player_account_changed_s {
    pub old_value: c_short,
    pub new_value: c_short,
}

#[repr(C)]
pub struct player_askedforball_s {
    pub userid: c_short,
}

#[repr(C)]
pub struct player_bonuspoints_s {
    pub points: c_short,
    pub player_entindex: c_short,
    pub source_entindex	: c_short,
}

#[repr(C)]
pub struct player_buff_s {
    pub userid: c_short,
    pub buff_owner: c_short,
    pub buff_type: c_short,
}

#[repr(C)]
pub struct player_builtobject_s {
    pub userid: c_short,
    pub object: c_short,
    pub index: c_short,
}

#[repr(C)]
pub struct player_buyback_s {
    pub player: c_short,
    pub cost: c_short,
}

#[repr(C)]
pub struct player_calledformedic_s {
    pub userid	: c_short,
}

#[repr(C)]
pub struct player_carryobject_s {
    pub userid: c_short,
    pub object: c_short,
    pub index: c_short,
}

#[repr(C)]
pub struct player_changeclass_s {
    pub userid: c_short,
    pub class: c_short,
}

#[repr(C)]
pub struct player_chargedeployed_s {
    pub userid: c_short,
    pub targetid: c_short,
}

#[repr(C)]
pub struct player_currency_changed_s {
    pub currency: c_short,
}

#[repr(C)]
pub struct player_damage_dodged_s {
    pub damage: c_short,
}

#[repr(C)]
pub struct player_damaged_s {
    pub amount: c_short,
    pub types: c_long,

}

#[repr(C)]
pub struct player_death_s {
    pub userid: c_short,
    pub victim_entindex: c_long,
    pub inflictor_entindex: c_long,
    pub attacker: c_short,
    pub weapon: c_char,
    pub weaponid: c_short,
    pub damagebits: c_long,
    pub customkill: c_short,
    pub assister: c_short,
    pub weapon_logclassname: c_char,
    pub stun_flags: c_short,
    pub death_flags: c_short,
    pub silent_kill: bool,
    pub playerpenetratecount: c_short,
    pub assister_fallback: c_char,
    pub kill_streak_total: c_short,
    pub kill_streak_wep: c_short,
    pub kill_streak_assist: c_short,
    pub kill_streak_victim: c_short,
    pub ducks_streaked: c_short,
    pub duck_streak_total: c_short,
    pub duck_streak_assist: c_short,
    pub duck_streak_victim: c_short,
    pub rocket_jump: bool,
    pub weapon_def_index: c_short,
    pub crit_type: c_short,

}

#[repr(C)]
pub struct player_destroyed_pipebomb_s {
    pub userid: c_short,
}

#[repr(C)]
pub struct player_directhit_stun_s {
    pub attacker: c_short,
    pub victim: c_short,
}

#[repr(C)]
pub struct player_domination_s {
    pub dominator: c_short,
    pub dominated: c_short,
    pub dominations: c_short,
}

#[repr(C)]
pub struct player_dropobject_s {
    pub userid: c_short,
    pub object: c_short,
    pub index: c_short,
}

#[repr(C)]
pub struct player_escort_score_s {
    pub player: c_short,
    pub points: c_short,
}

#[repr(C)]
pub struct player_extinguished_s {
    pub victim: c_short,
    pub healer: c_short,
    pub itemdefindex: c_short,
}

#[repr(C)]
pub struct player_healed_s {
    pub patient: c_short,
    pub healer	: c_short,
    pub amount: c_short,
}

#[repr(C)]
pub struct player_healedbymedic_s {
    pub medic: c_short,
}

#[repr(C)]
pub struct player_healedmediccall_s {
    pub userid: c_short,
}

#[repr(C)]
pub struct player_healonhit_s {
    pub amount: c_short,
    pub entindex: c_long,
}

#[repr(C)]
pub struct player_highfive_cancel_s {
    pub entindex: c_short,
}

#[repr(C)]
pub struct player_highfive_success_s {
    pub initiator_entindex: c_short,
    pub partner_entindex	: c_short,
}

#[repr(C)]
pub struct player_hurt_s {
    pub userid: c_short,
    pub health: c_short,
    pub attacker: c_short,
    pub damageamount: c_short,
    pub custom: c_short,
    pub showdisguisedcrit: bool,
    pub crit: bool,
    pub minicrit: bool,
    pub allseecrit: bool,
    pub weaponid: c_short,
    pub bonuseffect: c_short,
}

#[repr(C)]
pub struct player_ignited_inv_s {
    pub pyro_entindex: c_long,
    pub victim_entindex: c_long,
    pub medic_entindex: c_long,
}

#[repr(C)]
pub struct player_ignited_s {
    pub pyro_entindex: c_long,
    pub victim_entindex: c_long,
    pub weaponid: c_int,
}

#[repr(C)]
pub struct player_initial_spawn_s {
    pub index: c_short,
}

#[repr(C)]
pub struct player_invulned_s {
    pub userid: c_short,
    pub medic_userid: c_short,
}

#[repr(C)]
pub struct player_jarated_fade_s {
    pub thrower_entindex: c_long,
    pub victim_entindex: c_long,
}

#[repr(C)]
pub struct player_jarated_s {
    pub thrower_entindex: c_long,
    pub victim_entindex: c_long,
}

#[repr(C)]
pub struct player_killed_achievement_zone_s {
    pub attacker: c_short,
    pub victim: c_short,
    pub zone_id: c_short,
}

#[repr(C)]
pub struct player_mvp_s {
    pub player: c_short,
}

#[repr(C)]
pub struct player_next_map_vote_change_s {
    pub map_index: c_short,
    pub vote: c_short,
}

#[repr(C)]
pub struct player_pinned_s {
    pub pinned: c_short,
}

#[repr(C)]
pub struct player_rocketpack_pushed_s {
    pub pusher: c_short,
    pub pushed: c_short,
}

#[repr(C)]
pub struct player_sapped_object_s {
    pub userid: c_short,
    pub ownerid: c_short,
    pub object: c_short,
    pub sapperid: c_short,
}

#[repr(C)]
pub struct player_score_changed_s {
    pub player: c_short,
    pub delta: c_short,
}

#[repr(C)]
pub struct player_shield_blocked_s {
    pub attacker_entindex: c_long,
    pub blocker_entindex: c_long,
}

#[repr(C)]
pub struct player_spawn_s {
    pub userid: c_short,
    pub team: c_int,
    pub class: c_short,
}

#[repr(C)]
pub struct player_stats_updated_s {
    pub forceupload: bool,
}

#[repr(C)]
pub struct player_stealsandvich_s {
    pub owner: c_short,
    pub target: c_short,
}

#[repr(C)]
pub struct player_stunned_s {
    pub stunner: c_short,
    pub victim: c_short,
    pub victim_capping	: bool,
    pub big_stun: bool,
}

#[repr(C)]
pub struct player_teleported_s {
    pub userid: c_short,
    pub builderid: c_short,
    pub dist: c_float,
}

#[repr(C)]
pub struct player_turned_to_ghost_s {
    pub userid: c_short,
}

#[repr(C)]
pub struct player_upgradedobject_s {
    pub userid: c_short,
    pub object: c_short,
    pub index: c_short,
    pub isbuilder: bool,
}

#[repr(C)]
pub struct player_used_powerup_bottle_s {
    pub player: c_short,
    pub types: c_short,
    pub time: c_float,
}

#[repr(C)]
pub struct post_inventory_application_s {
    pub userid: c_short,
}

#[repr(C)]
pub struct projectile_direct_hit_s {
    pub attacker: c_short,
    pub victim: c_short,
    pub weapon_def_index: c_long,
}

#[repr(C)]
pub struct projectile_removed_s {
    pub attacker: c_short,
    pub weapon_def_index: c_long,
    pub num_hit: c_short,
    pub num_direct_hit: c_short,
}

#[repr(C)]
pub struct proto_def_changed_s {
    pub types: c_short,
    pub defindex: c_long,
    pub created: bool,
    pub deleted: bool,
    pub erase_history: bool,
}

#[repr(C)]
pub struct pve_win_panel_s {
    pub panel_style: c_short,
    pub winning_team: c_short,
    pub winreason: c_char,
}

#[repr(C)]
pub struct quest_objective_completed_s {
    pub quest_item_id_low: c_long,
    pub quest_item_id_hi: c_long,
    pub quest_objective_id: c_long,
    pub scorer_user_id: c_long,
}

#[repr(C)]
pub struct quest_progress_s {
    pub owner: c_short,
    pub scorer: c_short,
    pub types: c_short,
    pub completed: bool,
    pub quest_defindex: c_long,
}

#[repr(C)]
pub struct quest_request_s {
    pub request: c_long,
    pub msg: c_char,
}

#[repr(C)]
pub struct quest_response_s {
    pub request: c_long,
    pub success: bool,
    pub msg: c_char,
}

#[repr(C)]
pub struct quest_turn_in_state_s {
    pub state: c_short,
}

#[repr(C)]
pub struct rd_robot_impact_s {
    pub entindex: c_short,
    pub impulse_x: c_float,
    pub impulse_y: c_float,
    pub impulse_z: c_float,
}

#[repr(C)]
pub struct rd_robot_killed_s {
    pub userid: c_short,
    pub victim_entindex: c_long,
    pub inflictor_entindex: c_long,
    pub attacker: c_short,
    pub weapon: c_char,
    pub weaponid: c_short,
    pub damagebits: c_long,
    pub customkill: c_short,
    pub weapon_logclassname: c_char,
}

#[repr(C)]
pub struct rd_team_points_changed_s {
    pub points: c_short,
    pub team: c_short,
    pub method: c_short,
}

#[repr(C)]
pub struct rematch_vote_period_over_s {
    pub success: bool,
}

#[repr(C)]
pub struct remove_nemesis_relationships_s {
    pub player: c_short,
}

#[repr(C)]
pub struct respawn_ghost_s {
    pub reviver: c_short,
    pub ghost: c_short,
}

#[repr(C)]
pub struct restart_timer_time_s {
    pub time: c_char,
}

#[repr(C)]
pub struct revive_player_complete_s {
    pub entindex: c_short,
}

#[repr(C)]
pub struct revive_player_notify_s {
    pub entindex: c_short,
    pub marker_entindex: c_short,
}

#[repr(C)]
pub struct revive_player_stopped_s {
    pub entindex: c_short,
}

#[repr(C)]
pub struct rocket_jump_landed_s {
    pub userid: c_short,
}

#[repr(C)]
pub struct rocket_jump_s {
    pub userid: c_short,
    pub playsound: bool,
}

#[repr(C)]
pub struct rocketpack_landed_s {
    pub userid: c_short,
}

#[repr(C)]
pub struct rocketpack_launch_s {
    pub userid: c_short,
    pub playsound: bool,
}

#[repr(C)]
pub struct rps_taunt_event_s {
    pub winner: c_short,
    pub winner_rps: c_short,
    pub loser: c_short,
    pub loser_rps: c_short,
}

#[repr(C)]
pub struct scout_grand_slam_s {
    pub scout_id: c_short,
    pub target_id: c_short,
}

#[repr(C)]
pub struct scout_slamdoll_landed_s {
    pub target_index: c_short,
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

#[repr(C)]
pub struct sentry_on_go_active_s {
    pub index: c_short,
}

#[repr(C)]
pub struct show_annotation_s {
    pub worldPosX: c_float,
    pub worldPosY: c_float,
    pub worldPosZ: c_float,
    pub worldNormalX: c_float,
    pub worldNormalY: c_float,
    pub worldNormalZ: c_float,
    pub id: c_long,
    pub text: c_char,
    pub lifetime: c_float,
    pub visibilityBitfield: c_long,
    pub follow_entindex: c_long,
    pub show_distance: bool,
    pub play_sound: c_char,
    pub show_effect: bool,
}

#[repr(C)]
pub struct show_class_layout_s {
    pub show: bool,
}

#[repr(C)]
pub struct show_freezepanel_s {
    pub killer: c_short,
}

#[repr(C)]
pub struct show_vs_panel_s {
    pub show: bool,
}

#[repr(C)]
pub struct special_score_s {
    pub player: c_short,
}

#[repr(C)]
pub struct sticky_jump_landed_s {
    pub userid: c_short,
}

#[repr(C)]
pub struct sticky_jump_s {
    pub userid: c_short,
    pub playsound: bool,
}

#[repr(C)]
pub struct tagged_player_as_it_s {
    pub player: c_short,
}

#[repr(C)]
pub struct team_leader_killed_s {
    pub killer: c_short,
    pub victim: c_short,
}

#[repr(C)]
pub struct teamplay_alert_s {
    pub alert_type: c_short,
}

#[repr(C)]
pub struct teamplay_broadcast_audio_s {
    pub team: c_short,
    pub sound: c_char,
    pub additional_flags: c_short,
}

#[repr(C)]
pub struct teamplay_capture_blocked_s {
    pub cp: c_short,
    pub cpname: c_char,
    pub blocker: c_short,
    pub victim: c_short,
}

#[repr(C)]
pub struct teamplay_capture_broken_s {
    pub cp: c_short,
    pub cpname: c_char,
    pub time_remaining	: c_float,
}

#[repr(C)]
pub struct teamplay_flag_event_s {
    pub player: c_short,
    pub carrier: c_short,
    pub eventtype: c_short,
    pub home: c_short,
    pub team: c_short,
}

#[repr(C)]
pub struct teamplay_game_over_s {
    pub reason: c_char,
}

#[repr(C)]
pub struct teamplay_map_time_remaining_s {
    pub seconds: c_short,
}

#[repr(C)]
pub struct teamplay_point_captured_s {
    pub cp: c_short,
    pub cpname: c_char,
    pub team: c_short,
    pub cappers: c_char,
}

#[repr(C)]
pub struct teamplay_point_locked_s {
    pub cp: c_short,
    pub cpname: c_char,
    pub team: c_short,
}

#[repr(C)]
pub struct teamplay_point_startcapture_s {
    pub cp: c_short,
    pub cpname: c_char,
    pub team: c_short,
    pub capteam: c_short,
    pub cappers: c_char,
    pub captime: c_float,
}

#[repr(C)]
pub struct teamplay_point_unlocked_s {
    pub cp: c_short,
    pub cpname: c_char,
    pub 	team: c_short,
}

#[repr(C)]
pub struct teamplay_pre_round_time_left_s {
    pub time: c_short,
}

#[repr(C)]
pub struct teamplay_round_restart_seconds_s {
    pub seconds: c_short,
}

#[repr(C)]
pub struct teamplay_round_selected_s {
    pub round: c_char,
}

#[repr(C)]
pub struct teamplay_round_stalemate_s {
    pub reason: c_char,
}

#[repr(C)]
pub struct teamplay_round_start_s {
    pub full_reset: bool,
}

#[repr(C)]
pub struct teamplay_round_win_s {
    pub team: c_int,
    pub winreason: c_int,
    pub flagcaplimit: c_short,
    pub full_round: bool,
    pub round_time: c_float,
    pub losing_team_num_caps: c_short,
    pub was_sudden_death: bool,
}

#[repr(C)]
pub struct teamplay_team_ready_s {
    pub team: c_int,
}

#[repr(C)]
pub struct teamplay_teambalanced_player_s {
    pub player: c_short,
    pub team: c_short,
}

#[repr(C)]
pub struct teamplay_timer_flash_s {
    pub time_remaining: c_short,
}

#[repr(C)]
pub struct teamplay_timer_time_added_s {
    pub timer: c_short,
    pub seconds_added: c_short,
}

#[repr(C)]
pub struct teamplay_win_panel_s {
    pub panel_style: c_char,
    pub winning_team: c_short,
    pub winreason: c_char,
    pub cappers: c_char,
    pub flagcaplimit: c_short,
    pub blue_score: c_short,
    pub red_score: c_short,
    pub blue_score_prev: c_short,
    pub red_score_prev: c_short,
    pub round_complete: c_short,
    pub rounds_remaining: c_short,
    pub player_1: c_short,
    pub player_1_points: c_short,
    pub player_2: c_short,
    pub player_2_points: c_short,
    pub player_3: c_short,
    pub player_3_points: c_short,
    pub killstreak_player_1: c_short,
    pub killstreak_player_1_count: c_short,
    pub game_over: c_short,
}

#[repr(C)]
pub struct tf_game_over_s {
    pub reason: c_char,
}

#[repr(C)]
pub struct tf_map_time_remaining_s {
    pub seconds: c_long,
}

#[repr(C)]
pub struct tournament_stateupdate_s {
    pub userid: c_short,
    pub namechange: bool,
    pub readystate: c_short,
    pub newname: c_char,
}

#[repr(C)]
pub struct training_complete_s {
    pub next_map: c_char,
    pub map: c_char,
    pub text: c_char,
}

#[repr(C)]
pub struct update_status_item_s {
    pub index: c_short,
    pub object: c_short,
}

#[repr(C)]
pub struct upgrades_file_changed_s {
    pub path: c_char,
}