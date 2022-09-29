#[repr(C)]
pub struct achievement_earned_local {
    pub achievement: i32,

}

#[repr(C)]
pub struct achievement_earned {
    pub player: i32,
    pub achievement: i32,
}

#[repr(C)]
pub struct air_dash {
    pub player: i32,
}

#[repr(C)]
pub struct arena_match_maxstreak {
    pub team: i32,
    pub streak: i32,
}

#[repr(C)]
pub struct arena_player_notification {
    pub player: i32,
    pub message: String,
}

#[repr(C)]
pub struct arena_win_panel {
    pub paneltyle: i32,
    pub winning_team: i32,
    pub winreason: String,
    pub cappers: String,
    pub flagcaplimit: i32,
    pub bluecore: i32,
    pub redcore: i32,
    pub bluecore_prev: i32,
    pub redcore_prev: i32,
    pub round_complete: i32,
    pub player_1: i32,
    pub player_1_damage: i32,
    pub player_1_healing: i32,
    pub player_1_lifetime: i32,
    pub player_1_kills: i32,
    pub player_2: i32,
    pub player_2_damage: i32,
    pub player_2_healing: i32,
    pub player_2_lifetime: i32,
    pub player_2_kills: i32,
    pub player_3: i32,
    pub player_3_damage: i32,
    pub player_3_healing: i32,
    pub player_3_lifetime: i32,
    pub player_3_kills: i32,
    pub player_4: i32,
    pub player_4_damage: i32,
    pub player_4_healing: i32,
    pub player_4_lifetime: i32,
    pub player_4_kills: i32,
    pub player_5: i32,
    pub player_5_damage: i32,
    pub player_5_healing: i32,
    pub player_5_lifetime: i32,
    pub player_5_kills: i32,
    pub player_6: i32,
    pub player_6_damage: i32,
    pub player_6_healing: i32,
    pub player_6_lifetime: i32,
    pub player_6_kills: i32,
}

#[repr(C)]
pub struct arrow_impact {
    pub attachedEntity: i32,
    pub shooter: i32,
    pub boneIndexAttached: i32,
    pub bonePositionX: f32,
    pub bonePositionY: f32,
    pub bonePositionZ: f32,
    pub boneAnglesX: f32,
    pub boneAnglesY: f32,
    pub boneAnglesZ: f32,
    pub projectileType: i32,
    pub isCrit: bool,
}

#[repr(C)]
pub struct building_healed {
    pub building: i32,
    pub healer: i32,
    pub amount: i32,
}

#[repr(C)]
pub struct building_info_changed {
    pub building_type: i32,
    pub object_mode	: i32,
    pub remove: i32,
}

#[repr(C)]
pub struct capper_killed {
    pub blocker: i32,
    pub victim: i32,

}

#[repr(C)]
pub struct christmas_gift_grab {
    pub userid: i32,
}

#[repr(C)]
pub struct cl_drawline {
    pub player: i32,
    pub panel: i32,
    pub line: i32,
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
pub struct competitivetats_update {
    pub index: i32,
    pub kills_rank: i32,
    pub score_rank: i32,
    pub damage_rank: i32,
    pub healing_rank: i32,
    pub support_rank: i32,
}

#[repr(C)]
pub struct conga_kill {
    pub index: i32,
}

#[repr(C)]
pub struct controlpoi32_endtouch {
    pub player: i32,
    pub area: i32,
}

#[repr(C)]
pub struct controlpoi32_fake_capture_mult {
    pub player: i32,
    pub i32_data: i32,
}

#[repr(C)]
pub struct controlpoi32_pulse_element {
    pub player: i32,
}

#[repr(C)]
pub struct controlpoi32tarttouch {
    pub player: i32,
    pub area: i32,
}

#[repr(C)]
pub struct controlpoi32_timer_updated_s {
    pub entindex: i32,
    pub time: f32,
}

#[repr(C)]
pub struct controlpoi32_unlock_updated_s {
    pub index: i32,
    pub time: f32,
}

#[repr(C)]
pub struct controlpoi32_updatecapping_s {
    pub index: i32,
}

#[repr(C)]
pub struct controlpoi32_updateimages_s {
    pub index: i32,
}

#[repr(C)]
pub struct controlpoi32_updateowner_s {
    pub index: i32,
}

#[repr(C)]
pub struct cross_spectral_bridge_s {
    pub player: i32,
}

#[repr(C)]
pub struct crossbow_heal_s {
    pub healer: i32,
    pub target: i32,
    pub amount: i32,
}

#[repr(C)]
pub struct ctf_flag_captured_s {
    pub capping_team: i32,
    pub capping_team_score	: i32,
}

#[repr(C)]
pub struct damage_mitigated_s {
    pub mitigator: i32,
    pub damaged: i32,
    pub amount: i32,
    pub itemdefindex: i32,
}

#[repr(C)]
pub struct damage_prevented_s {
    pub preventor: i32,
    pub victim: i32,
    pub amount: i32,
    pub condition: i32,
}

#[repr(C)]
pub struct damage_resisted_s {
    pub entindex: i32,
}

#[repr(C)]
pub struct demoman_det_stickies_s {
    pub player: i32,
}

#[repr(C)]
pub struct deploy_buff_banner_s {
    pub buff_type: i32,
    pub buff_owner: i32,
}

#[repr(C)]
pub struct doomsday_rocket_open_s {
    pub team: i32,
}

#[repr(C)]
pub struct duel_status_s {
    pub killer: i32,
    pub score_type: i32,
    pub initiator: i32,
    pub target: i32,
    pub initiator_score: i32,
    pub target_score: i32,
}

#[repr(C)]
pub struct environmental_death_s {
    pub killer: i32,
    pub victim: i32,
}

#[repr(C)]
pub struct escaped_hell_s {
    pub player: i32,
}

#[repr(C)]
pub struct escaped_loot_island_s {
    pub player: i32,
}

#[repr(C)]
pub struct escort_progress_s {
    pub team: i32,
    pub progress: f32,
    pub reset: bool,
}

#[repr(C)]
pub struct escort_recede_s {
    pub team: i32,
    pub recedetime: f32,
}

#[repr(C)]
pub struct escort_speed_s {
    pub team: i32,
    pub speed: String,
    pub players: i32,
}

#[repr(C)]
pub struct eyeball_boss_escape_imminent_s {
    pub level: i32,
    pub time_remaining: String,
}

#[repr(C)]
pub struct eyeball_boss_escaped_s {
    pub level: i32,

}

#[repr(C)]
pub struct eyeball_boss_killed_s {
    pub level: i32,
}

#[repr(C)]
pub struct eyeball_boss_killer_s {
    pub level: i32,
    pub player_entindex: i32,
}

#[repr(C)]
pub struct eyeball_boss_stunned_s {
    pub level: i32,
    pub player_entindex: i32,
}

#[repr(C)]
pub struct fish_notice_s {
    pub userid: i32,
    pub victim_entindex: i32,
    pub inflictor_entindex: i32,
    pub attacker: i32,
    pub weapon: String,
    pub weaponid: i32,
    pub damagebits: i32,
    pub customkill: i32,
    pub assister: i32,
    pub weapon_logclassname: String,
    pub stun_flags: i32,
    pub death_flags: i32,
    pub silent_kill: bool,
    pub assister_fallback: String,
}

#[repr(C)]
pub struct flagstatus_update_s {
    pub userid: i32,
    pub entindex: i32,
}

#[repr(C)]
pub struct gas_doused_player_ignited_s {
    pub igniter: i32,
    pub douser: i32,
    pub victim: i32,
}

#[repr(C)]
pub struct halloween_boss_killed_s {
    pub boss: i32,
    pub killer: i32,
}

#[repr(C)]
pub struct halloween_duck_collected_s {
    pub collector: i32,
}

#[repr(C)]
pub struct halloween_pumpkin_grab_s {
    pub userid: i32,
}

#[repr(C)]
pub struct halloween_skeleton_killed_s {
    pub player: i32,
}

#[repr(C)]
pub struct halloween_soul_collected_s {
    pub i32ended_target: i32,
    pub collecting_player: i32,
    pub soul_count: i32,
}

#[repr(C)]
pub struct hide_annotation_s {
    pub id: i32,
}

#[repr(C)]
pub struct i32ro_finish_s {
    pub player: i32,
}

#[repr(C)]
pub struct i32ro_nextcamera_s {
    pub player: i32,
}

#[repr(C)]
pub struct item_found_s {
    pub player: i32,
    pub quality: i32,
    pub method: i32,
    pub itemdef: i32,
    pub isstrange: i32,
    pub isunusual: i32,
    pub wear: f32,
}

#[repr(C)]
pub struct item_pickup_s {
    pub userid: i32,
    pub item: String,
}

#[repr(C)]
pub struct kill_in_hell_s {
    pub killer: i32,
    pub victim: i32,
}

#[repr(C)]
pub struct kill_refills_meter_s {
    pub index: i32,
}

#[repr(C)]
pub struct killed_capping_player_s {
    pub cp: i32,
    pub killer: i32,
    pub victim: i32,
    pub assister: i32,
}

#[repr(C)]
pub struct landed_s {
    pub player: i32,
}

#[repr(C)]
pub struct localplayer_changedisguise_s {
    pub disguised: bool,
}

#[repr(C)]
pub struct localplayer_healed_s {
    pub amount: i32,
}

#[repr(C)]
pub struct localplayer_score_changed_s {
    pub score: i32,
}

#[repr(C)]
pub struct medic_death_s {
    pub userid: i32,
    pub attacker: i32,
    pub healing: i32,
    pub charged: bool,
}

#[repr(C)]
pub struct medigun_shield_blocked_damage_s {
    pub userid: i32,
    pub damage: f32,
}

#[repr(C)]
pub struct merasmus_escape_warning_s {
    pub level: i32,
    pub time_remaining: String,
}

#[repr(C)]
pub struct merasmus_escaped_s {
    pub level: i32,
}

#[repr(C)]
pub struct merasmus_prop_found_s {
    pub player: i32,
}

#[repr(C)]
pub struct merasmus_stunned_s {
    pub player: i32,
}

#[repr(C)]
pub struct minigame_win_s {
    pub team: i32,
    pub types: i32,
}

#[repr(C)]
pub struct minigame_won_s {
    pub player: i32,
    pub game: i32,
}

#[repr(C)]
pub struct mvm_adv_wave_complete_no_gates_s {
    pub index: i32,
}

#[repr(C)]
pub struct mvm_begin_wave_s {
    pub wave_index: i32,
    pub max_waves: i32,
    pub advanced: i32,
}

#[repr(C)]
pub struct mvm_bomb_carrier_killed_s {
    pub level: i32,
}

#[repr(C)]
pub struct mvm_bomb_deploy_reset_by_player_s {
    pub player: i32,
}

#[repr(C)]
pub struct mvm_bomb_reset_by_player_s {
    pub player: i32,
}

#[repr(C)]
pub struct mvm_kill_robot_delivering_bomb_s {
    pub player: i32,
}

#[repr(C)]
pub struct mvm_medic_powerup_shared_s {
    pub player: i32,
}

#[repr(C)]
pub struct mvm_mission_complete_s {
    pub mission: String,
}

#[repr(C)]
pub struct mvm_mission_update_s {
    pub class	: i32,
    pub count: i32,
}

#[repr(C)]
pub struct mvm_pickup_currency_s {
    pub player: i32,
    pub currency: i32,
}

#[repr(C)]
pub struct mvm_quick_sentry_upgrade_s {
    pub player: i32,
}

#[repr(C)]
pub struct mvm_scout_marked_for_death_s {
    pub player: i32,
}

#[repr(C)]
pub struct mvm_sentrybuster_detonate_s {
    pub player: i32,
    pub det_x: f32,
    pub det_y: f32,
    pub det_z: f32,
}

#[repr(C)]
pub struct mvm_sentrybuster_killed_s {
    pub sentry_buster: i32,
}

#[repr(C)]
pub struct mvm_sniper_headshot_currency_s {
    pub userid: i32,
    pub currency: i32,
}

#[repr(C)]
pub struct mvm_wave_complete_s {
    pub advanced: bool,
}

#[repr(C)]
pub struct nav_blocked_s {
    pub area: i32,
    pub blocked: bool,
}

#[repr(C)]
pub struct num_cappers_changed_s {
    pub index: i32,
    pub count: i32,
}

#[repr(C)]
pub struct object_deflected_s {
    pub userid: i32,
    pub ownerid: i32,
    pub weaponid: i32,
    pub object_entindex: i32,
}

#[repr(C)]
pub struct object_destroyed_s {
    pub userid: i32,
    pub attacker: i32,
    pub assister: i32,
    pub weapon: String,
    pub weaponid: i32,
    pub objecttype: i32,
    pub index: i32,
    pub was_building: bool,
}

#[repr(C)]
pub struct object_detonated_s {
    pub userid: i32,
    pub objecttype: i32,
    pub index: i32,
}

#[repr(C)]
pub struct object_removed_s {
    pub userid: i32,
    pub objecttype: i32,
    pub index: i32,
}

#[repr(C)]
pub struct parachute_deploy_s {
    pub index: i32,
}

#[repr(C)]
pub struct parachute_holster_s {
    pub index: i32,
}

#[repr(C)]
pub struct party_chat_s {
    pub steamid: String,
    pub text: String,
    pub types: i32,
}

#[repr(C)]
pub struct party_member_join_s {
    pub steamid: String,
}

#[repr(C)]
pub struct party_member_leave_s {
    pub steamid: String,
}

#[repr(C)]
pub struct pass_ball_blocked_s {
    pub owner: i32,
    pub blocker: i32,
}

#[repr(C)]
pub struct pass_ball_stolen_s {
    pub victim: i32,
    pub attacker: i32,
}

#[repr(C)]
pub struct pass_free_s {
    pub owner: i32,
    pub attacker: i32,
}

#[repr(C)]
pub struct pass_get_s {
    pub owner: i32,
}

#[repr(C)]
pub struct pass_pass_caught_s {
    pub passer: i32,
    pub catcher: i32,
    pub dist: f32,
    pub duration: f32,
}

#[repr(C)]
pub struct pass_score_s {
    pub scorer: i32,
    pub assister: i32,
    pub poi32s: i32,
}

#[repr(C)]
pub struct path_track_passed_s {
    pub index: i32,
}

#[repr(C)]
pub struct payload_pushed_s {
    pub pusher: i32,
    pub distance: String,
}

#[repr(C)]
pub struct player_abandoned_match_s {
    pub game_over: bool,
}

#[repr(C)]
pub struct player_account_changed_s {
    pub old_value: i32,
    pub new_value: i32,
}

#[repr(C)]
pub struct player_askedforball_s {
    pub userid: i32,
}

#[repr(C)]
pub struct player_bonuspoi32s_s {
    pub poi32s: i32,
    pub player_entindex: i32,
    pub source_entindex	: i32,
}

#[repr(C)]
pub struct player_buff_s {
    pub userid: i32,
    pub buff_owner: i32,
    pub buff_type: i32,
}

#[repr(C)]
pub struct player_builtobject_s {
    pub userid: i32,
    pub object: i32,
    pub index: i32,
}

#[repr(C)]
pub struct player_buyback_s {
    pub player: i32,
    pub cost: i32,
}

#[repr(C)]
pub struct player_calledformedic_s {
    pub userid	: i32,
}

#[repr(C)]
pub struct player_carryobject_s {
    pub userid: i32,
    pub object: i32,
    pub index: i32,
}

#[repr(C)]
pub struct player_changeclass_s {
    pub userid: i32,
    pub class: i32,
}

#[repr(C)]
pub struct player_chargedeployed_s {
    pub userid: i32,
    pub targetid: i32,
}

#[repr(C)]
pub struct player_currency_changed_s {
    pub currency: i32,
}

#[repr(C)]
pub struct player_damage_dodged_s {
    pub damage: i32,
}

#[repr(C)]
pub struct player_damaged_s {
    pub amount: i32,
    pub types: i32,

}

#[repr(C)]
pub struct player_death_s {
    pub userid: i32,
    pub victim_entindex: i32,
    pub inflictor_entindex: i32,
    pub attacker: i32,
    pub weapon: String,
    pub weaponid: i32,
    pub damagebits: i32,
    pub customkill: i32,
    pub assister: i32,
    pub weapon_logclassname: String,
    pub stun_flags: i32,
    pub death_flags: i32,
    pub silent_kill: bool,
    pub playerpenetratecount: i32,
    pub assister_fallback: String,
    pub kill_streak_total: i32,
    pub kill_streak_wep: i32,
    pub kill_streak_assist: i32,
    pub kill_streak_victim: i32,
    pub ducks_streaked: i32,
    pub duck_streak_total: i32,
    pub duck_streak_assist: i32,
    pub duck_streak_victim: i32,
    pub rocket_jump: bool,
    pub weapon_def_index: i32,
    pub crit_type: i32,

}

#[repr(C)]
pub struct player_destroyed_pipebomb_s {
    pub userid: i32,
}

#[repr(C)]
pub struct player_directhit_stun_s {
    pub attacker: i32,
    pub victim: i32,
}

#[repr(C)]
pub struct player_domination_s {
    pub dominator: i32,
    pub dominated: i32,
    pub dominations: i32,
}

#[repr(C)]
pub struct player_dropobject_s {
    pub userid: i32,
    pub object: i32,
    pub index: i32,
}

#[repr(C)]
pub struct player_escort_score_s {
    pub player: i32,
    pub poi32s: i32,
}

#[repr(C)]
pub struct player_extinguished_s {
    pub victim: i32,
    pub healer: i32,
    pub itemdefindex: i32,
}

#[repr(C)]
pub struct player_healed_s {
    pub patient: i32,
    pub healer	: i32,
    pub amount: i32,
}

#[repr(C)]
pub struct player_healedbymedic_s {
    pub medic: i32,
}

#[repr(C)]
pub struct player_healedmediccall_s {
    pub userid: i32,
}

#[repr(C)]
pub struct player_healonhit_s {
    pub amount: i32,
    pub entindex: i32,
}

#[repr(C)]
pub struct player_highfive_cancel_s {
    pub entindex: i32,
}

#[repr(C)]
pub struct player_highfive_success_s {
    pub initiator_entindex: i32,
    pub partner_entindex	: i32,
}

#[repr(C)]
pub struct player_hurt_s {
    pub userid: i32,
    pub health: i32,
    pub attacker: i32,
    pub damageamount: i32,
    pub custom: i32,
    pub showdisguisedcrit: bool,
    pub crit: bool,
    pub minicrit: bool,
    pub allseecrit: bool,
    pub weaponid: i32,
    pub bonuseffect: i32,
}

#[repr(C)]
pub struct player_ignited_inv_s {
    pub pyro_entindex: i32,
    pub victim_entindex: i32,
    pub medic_entindex: i32,
}

#[repr(C)]
pub struct player_ignited_s {
    pub pyro_entindex: i32,
    pub victim_entindex: i32,
    pub weaponid: i32,
}

#[repr(C)]
pub struct player_initial_spawn_s {
    pub index: i32,
}

#[repr(C)]
pub struct player_invulned_s {
    pub userid: i32,
    pub medic_userid: i32,
}

#[repr(C)]
pub struct player_jarated_fade_s {
    pub thrower_entindex: i32,
    pub victim_entindex: i32,
}

#[repr(C)]
pub struct player_jarated_s {
    pub thrower_entindex: i32,
    pub victim_entindex: i32,
}

#[repr(C)]
pub struct player_killed_achievement_zone_s {
    pub attacker: i32,
    pub victim: i32,
    pub zone_id: i32,
}

#[repr(C)]
pub struct player_mvp_s {
    pub player: i32,
}

#[repr(C)]
pub struct player_next_map_vote_change_s {
    pub map_index: i32,
    pub vote: i32,
}

#[repr(C)]
pub struct player_pinned_s {
    pub pinned: i32,
}

#[repr(C)]
pub struct player_rocketpack_pushed_s {
    pub pusher: i32,
    pub pushed: i32,
}

#[repr(C)]
pub struct player_sapped_object_s {
    pub userid: i32,
    pub ownerid: i32,
    pub object: i32,
    pub sapperid: i32,
}

#[repr(C)]
pub struct player_score_changed_s {
    pub player: i32,
    pub delta: i32,
}

#[repr(C)]
pub struct player_shield_blocked_s {
    pub attacker_entindex: i32,
    pub blocker_entindex: i32,
}

#[repr(C)]
pub struct player_spawn_s {
    pub userid: i32,
    pub team: i32,
    pub class: i32,
}

#[repr(C)]
pub struct player_stats_updated_s {
    pub forceupload: bool,
}

#[repr(C)]
pub struct player_stealsandvich_s {
    pub owner: i32,
    pub target: i32,
}

#[repr(C)]
pub struct player_stunned_s {
    pub stunner: i32,
    pub victim: i32,
    pub victim_capping	: bool,
    pub big_stun: bool,
}

#[repr(C)]
pub struct player_teleported_s {
    pub userid: i32,
    pub builderid: i32,
    pub dist: f32,
}

#[repr(C)]
pub struct player_turned_to_ghost_s {
    pub userid: i32,
}

#[repr(C)]
pub struct player_upgradedobject_s {
    pub userid: i32,
    pub object: i32,
    pub index: i32,
    pub isbuilder: bool,
}

#[repr(C)]
pub struct player_used_powerup_bottle_s {
    pub player: i32,
    pub types: i32,
    pub time: f32,
}

#[repr(C)]
pub struct post_inventory_application_s {
    pub userid: i32,
}

#[repr(C)]
pub struct projectile_direct_hit_s {
    pub attacker: i32,
    pub victim: i32,
    pub weapon_def_index: i32,
}

#[repr(C)]
pub struct projectile_removed_s {
    pub attacker: i32,
    pub weapon_def_index: i32,
    pub num_hit: i32,
    pub num_direct_hit: i32,
}

#[repr(C)]
pub struct proto_def_changed_s {
    pub types: i32,
    pub defindex: i32,
    pub created: bool,
    pub deleted: bool,
    pub erase_history: bool,
}

#[repr(C)]
pub struct pve_win_panel_s {
    pub panel_style: i32,
    pub winning_team: i32,
    pub winreason: String,
}

#[repr(C)]
pub struct quest_objective_completed_s {
    pub quest_item_id_low: i32,
    pub quest_item_id_hi: i32,
    pub quest_objective_id: i32,
    pub scorer_user_id: i32,
}

#[repr(C)]
pub struct quest_progress_s {
    pub owner: i32,
    pub scorer: i32,
    pub types: i32,
    pub completed: bool,
    pub quest_defindex: i32,
}

#[repr(C)]
pub struct quest_request_s {
    pub request: i32,
    pub msg: String,
}

#[repr(C)]
pub struct quest_response_s {
    pub request: i32,
    pub success: bool,
    pub msg: String,
}

#[repr(C)]
pub struct quest_turn_in_state_s {
    pub state: i32,
}

#[repr(C)]
pub struct rd_robot_impact_s {
    pub entindex: i32,
    pub impulse_x: f32,
    pub impulse_y: f32,
    pub impulse_z: f32,
}

#[repr(C)]
pub struct rd_robot_killed_s {
    pub userid: i32,
    pub victim_entindex: i32,
    pub inflictor_entindex: i32,
    pub attacker: i32,
    pub weapon: String,
    pub weaponid: i32,
    pub damagebits: i32,
    pub customkill: i32,
    pub weapon_logclassname: String,
}

#[repr(C)]
pub struct rd_team_poi32s_changed_s {
    pub poi32s: i32,
    pub team: i32,
    pub method: i32,
}

#[repr(C)]
pub struct rematch_vote_period_over_s {
    pub success: bool,
}

#[repr(C)]
pub struct remove_nemesis_relationships_s {
    pub player: i32,
}

#[repr(C)]
pub struct respawn_ghost_s {
    pub reviver: i32,
    pub ghost: i32,
}

#[repr(C)]
pub struct restart_timer_time_s {
    pub time: String,
}

#[repr(C)]
pub struct revive_player_complete_s {
    pub entindex: i32,
}

#[repr(C)]
pub struct revive_player_notify_s {
    pub entindex: i32,
    pub marker_entindex: i32,
}

#[repr(C)]
pub struct revive_player_stopped_s {
    pub entindex: i32,
}

#[repr(C)]
pub struct rocket_jump_landed_s {
    pub userid: i32,
}

#[repr(C)]
pub struct rocket_jump_s {
    pub userid: i32,
    pub playsound: bool,
}

#[repr(C)]
pub struct rocketpack_landed_s {
    pub userid: i32,
}

#[repr(C)]
pub struct rocketpack_launch_s {
    pub userid: i32,
    pub playsound: bool,
}

#[repr(C)]
pub struct rps_taunt_event_s {
    pub winner: i32,
    pub winner_rps: i32,
    pub loser: i32,
    pub loser_rps: i32,
}

#[repr(C)]
pub struct scout_grand_slam_s {
    pub scout_id: i32,
    pub target_id: i32,
}

#[repr(C)]
pub struct scout_slamdoll_landed_s {
    pub target_index: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct sentry_on_go_active_s {
    pub index: i32,
}

#[repr(C)]
pub struct show_annotation_s {
    pub worldPosX: f32,
    pub worldPosY: f32,
    pub worldPosZ: f32,
    pub worldNormalX: f32,
    pub worldNormalY: f32,
    pub worldNormalZ: f32,
    pub id: i32,
    pub text: String,
    pub lifetime: f32,
    pub visibilityBitfield: i32,
    pub follow_entindex: i32,
    pub show_distance: bool,
    pub play_sound: String,
    pub show_effect: bool,
}

#[repr(C)]
pub struct show_class_layout_s {
    pub show: bool,
}

#[repr(C)]
pub struct show_freezepanel_s {
    pub killer: i32,
}

#[repr(C)]
pub struct show_vs_panel_s {
    pub show: bool,
}

#[repr(C)]
pub struct special_score_s {
    pub player: i32,
}

#[repr(C)]
pub struct sticky_jump_landed_s {
    pub userid: i32,
}

#[repr(C)]
pub struct sticky_jump_s {
    pub userid: i32,
    pub playsound: bool,
}

#[repr(C)]
pub struct tagged_player_as_it_s {
    pub player: i32,
}

#[repr(C)]
pub struct team_leader_killed_s {
    pub killer: i32,
    pub victim: i32,
}

#[repr(C)]
pub struct teamplay_alert_s {
    pub alert_type: i32,
}

#[repr(C)]
pub struct teamplay_broadcast_audio_s {
    pub team: i32,
    pub sound: String,
    pub additional_flags: i32,
}

#[repr(C)]
pub struct teamplay_capture_blocked_s {
    pub cp: i32,
    pub cpname: String,
    pub blocker: i32,
    pub victim: i32,
}

#[repr(C)]
pub struct teamplay_capture_broken_s {
    pub cp: i32,
    pub cpname: String,
    pub time_remaining	: f32,
}

#[repr(C)]
pub struct teamplay_flag_event_s {
    pub player: i32,
    pub carrier: i32,
    pub eventtype: i32,
    pub home: i32,
    pub team: i32,
}

#[repr(C)]
pub struct teamplay_game_over_s {
    pub reason: String,
}

#[repr(C)]
pub struct teamplay_map_time_remaining_s {
    pub seconds: i32,
}

#[repr(C)]
pub struct teamplay_poi32_captured_s {
    pub cp: i32,
    pub cpname: String,
    pub team: i32,
    pub cappers: String,
}

#[repr(C)]
pub struct teamplay_poi32_locked_s {
    pub cp: i32,
    pub cpname: String,
    pub team: i32,
}

#[repr(C)]
pub struct teamplay_poi32_startcapture_s {
    pub cp: i32,
    pub cpname: String,
    pub team: i32,
    pub capteam: i32,
    pub cappers: String,
    pub captime: f32,
}

#[repr(C)]
pub struct teamplay_poi32_unlocked_s {
    pub cp: i32,
    pub cpname: String,
    pub 	team: i32,
}

#[repr(C)]
pub struct teamplay_pre_round_time_left_s {
    pub time: i32,
}

#[repr(C)]
pub struct teamplay_round_restart_seconds_s {
    pub seconds: i32,
}

#[repr(C)]
pub struct teamplay_round_selected_s {
    pub round: String,
}

#[repr(C)]
pub struct teamplay_round_stalemate_s {
    pub reason: String,
}

#[repr(C)]
pub struct teamplay_round_start_s {
    pub full_reset: bool,
}

#[repr(C)]
pub struct teamplay_round_win_s {
    pub team: i32,
    pub winreason: i32,
    pub flagcaplimit: i32,
    pub full_round: bool,
    pub round_time: f32,
    pub losing_team_num_caps: i32,
    pub was_sudden_death: bool,
}

#[repr(C)]
pub struct teamplay_team_ready_s {
    pub team: i32,
}

#[repr(C)]
pub struct teamplay_teambalanced_player_s {
    pub player: i32,
    pub team: i32,
}

#[repr(C)]
pub struct teamplay_timer_flash_s {
    pub time_remaining: i32,
}

#[repr(C)]
pub struct teamplay_timer_time_added_s {
    pub timer: i32,
    pub seconds_added: i32,
}

#[repr(C)]
pub struct teamplay_win_panel_s {
    pub panel_style: String,
    pub winning_team: i32,
    pub winreason: String,
    pub cappers: String,
    pub flagcaplimit: i32,
    pub blue_score: i32,
    pub red_score: i32,
    pub blue_score_prev: i32,
    pub red_score_prev: i32,
    pub round_complete: i32,
    pub rounds_remaining: i32,
    pub player_1: i32,
    pub player_1_poi32s: i32,
    pub player_2: i32,
    pub player_2_poi32s: i32,
    pub player_3: i32,
    pub player_3_poi32s: i32,
    pub killstreak_player_1: i32,
    pub killstreak_player_1_count: i32,
    pub game_over: i32,
}

#[repr(C)]
pub struct tf_game_over_s {
    pub reason: String,
}

#[repr(C)]
pub struct tf_map_time_remaining_s {
    pub seconds: i32,
}

#[repr(C)]
pub struct tournament_stateupdate_s {
    pub userid: i32,
    pub namechange: bool,
    pub readystate: i32,
    pub newname: String,
}

#[repr(C)]
pub struct training_complete_s {
    pub next_map: String,
    pub map: String,
    pub text: String,
}

#[repr(C)]
pub struct update_status_item_s {
    pub index: i32,
    pub object: i32,
}

#[repr(C)]
pub struct upgrades_file_changed_s {
    pub path: String,
}