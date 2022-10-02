
pub struct achievement_earned_local {
    pub achievement: i32,

}


pub struct achievement_earned {
    pub player: i32,
    pub achievement: i32,
}


pub struct air_dash {
    pub player: i32,
}


pub struct arena_match_maxstreak {
    pub team: i32,
    pub streak: i32,
}


pub struct arena_player_notification {
    pub player: i32,
    pub message: String,
}


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


pub struct building_healed {
    pub building: i32,
    pub healer: i32,
    pub amount: i32,
}


pub struct building_info_changed {
    pub building_type: i32,
    pub object_mode	: i32,
    pub remove: i32,
}


pub struct capper_killed {
    pub blocker: i32,
    pub victim: i32,

}


pub struct christmas_gift_grab {
    pub userid: i32,
}


pub struct cl_drawline {
    pub player: i32,
    pub panel: i32,
    pub line: i32,
    pub x: f32,
    pub y: f32,
}


pub struct competitivetats_update {
    pub index: i32,
    pub kills_rank: i32,
    pub score_rank: i32,
    pub damage_rank: i32,
    pub healing_rank: i32,
    pub support_rank: i32,
}


pub struct conga_kill {
    pub index: i32,
}


pub struct controlpoint_endtouch {
    pub player: i32,
    pub area: i32,
}


pub struct controlpoint_fake_capture_mult {
    pub player: i32,
    pub i32_data: i32,
}


pub struct controlpoint_pulse_element {
    pub player: i32,
}


pub struct controlpointtarttouch {
    pub player: i32,
    pub area: i32,
}


pub struct controlpoint_timer_updated {
    pub entindex: i32,
    pub time: f32,
}


pub struct controlpoint_unlock_updated {
    pub index: i32,
    pub time: f32,
}


pub struct controlpoint_updatecapping {
    pub index: i32,
}


pub struct controlpoint_updateimages {
    pub index: i32,
}


pub struct controlpoint_updatelayout {
    pub index: i32,
}


pub struct controlpoint_updateowner {
    pub index: i32,
}


pub struct cross_spectral_bridge {
    pub player: i32,
}


pub struct crossbow_heal {
    pub healer: i32,
    pub target: i32,
    pub amount: i32,
}


pub struct ctf_flag_captured {
    pub capping_team: i32,
    pub capping_team_score	: i32,
}


pub struct damage_mitigated {
    pub mitigator: i32,
    pub damaged: i32,
    pub amount: i32,
    pub itemdefindex: i32,
}


pub struct damage_prevented {
    pub preventor: i32,
    pub victim: i32,
    pub amount: i32,
    pub condition: i32,
}


pub struct damage_resisted {
    pub entindex: i32,
}


pub struct demoman_det_stickies {
    pub player: i32,
}


pub struct deploy_buff_banner {
    pub buff_type: i32,
    pub buff_owner: i32,
}


pub struct doomsday_rocket_open {
    pub team: i32,
}


pub struct duel_status {
    pub killer: i32,
    pub score_type: i32,
    pub initiator: i32,
    pub target: i32,
    pub initiator_score: i32,
    pub target_score: i32,
}


pub struct environmental_death {
    pub killer: i32,
    pub victim: i32,
}


pub struct escaped_hell {
    pub player: i32,
}


pub struct escaped_loot_island {
    pub player: i32,
}


pub struct escort_progress {
    pub team: i32,
    pub progress: f32,
    pub reset: bool,
}


pub struct escort_recede {
    pub team: i32,
    pub recedetime: f32,
}


pub struct escort_speed {
    pub team: i32,
    pub speed: String,
    pub players: i32,
}


pub struct eyeball_boss_escape_imminent {
    pub level: i32,
    pub time_remaining: String,
}


pub struct eyeball_boss_escaped {
    pub level: i32,

}


pub struct eyeball_boss_killed {
    pub level: i32,
}


pub struct eyeball_boss_killer {
    pub level: i32,
    pub player_entindex: i32,
}


pub struct eyeball_boss_stunned {
    pub level: i32,
    pub player_entindex: i32,
}


pub struct fish_notice {
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


pub struct flagstatus_update {
    pub userid: i32,
    pub entindex: i32,
}


pub struct gas_doused_player_ignited {
    pub igniter: i32,
    pub douser: i32,
    pub victim: i32,
}


pub struct halloween_boss_killed {
    pub boss: i32,
    pub killer: i32,
}


pub struct halloween_duck_collected {
    pub collector: i32,
}


pub struct halloween_pumpkin_grab {
    pub userid: i32,
}


pub struct halloween_skeleton_killed {
    pub player: i32,
}


pub struct halloween_soul_collected {
    pub i32ended_target: i32,
    pub collecting_player: i32,
    pub soul_count: i32,
}


pub struct hide_annotation {
    pub id: i32,
}


pub struct i32ro_finish {
    pub player: i32,
}


pub struct i32ro_nextcamera {
    pub player: i32,
}


pub struct item_found {
    pub player: i32,
    pub quality: i32,
    pub method: i32,
    pub itemdef: i32,
    pub isstrange: i32,
    pub isunusual: i32,
    pub wear: f32,
}


pub struct item_pickup {
    pub userid: i32,
    pub item: String,
}


pub struct kill_in_hell {
    pub killer: i32,
    pub victim: i32,
}


pub struct kill_refills_meter {
    pub index: i32,
}


pub struct killed_capping_player {
    pub cp: i32,
    pub killer: i32,
    pub victim: i32,
    pub assister: i32,
}


pub struct landed {
    pub player: i32,
}


pub struct localplayer_changedisguise {
    pub disguised: bool,
}


pub struct localplayer_healed {
    pub amount: i32,
}


pub struct localplayer_score_changed {
    pub score: i32,
}


pub struct medic_death {
    pub userid: i32,
    pub attacker: i32,
    pub healing: i32,
    pub charged: bool,
}


pub struct medigun_shield_blocked_damage {
    pub userid: i32,
    pub damage: f32,
}


pub struct merasmus_escape_warning {
    pub level: i32,
    pub time_remaining: String,
}


pub struct merasmus_escaped {
    pub level: i32,
}


pub struct merasmus_prop_found {
    pub player: i32,
}


pub struct merasmus_stunned {
    pub player: i32,
}


pub struct minigame_win {
    pub team: i32,
    pub types: i32,
}


pub struct minigame_won {
    pub player: i32,
    pub game: i32,
}


pub struct mvm_adv_wave_complete_no_gates {
    pub index: i32,
}


pub struct mvm_begin_wave {
    pub wave_index: i32,
    pub max_waves: i32,
    pub advanced: i32,
}


pub struct mvm_bomb_carrier_killed {
    pub level: i32,
}


pub struct mvm_bomb_deploy_reset_by_player {
    pub player: i32,
}


pub struct mvm_bomb_reset_by_player {
    pub player: i32,
}


pub struct mvm_kill_robot_delivering_bomb {
    pub player: i32,
}


pub struct mvm_medic_powerup_shared {
    pub player: i32,
}


pub struct mvm_mission_complete {
    pub mission: String,
}


pub struct mvm_mission_update {
    pub class	: i32,
    pub count: i32,
}


pub struct mvm_pickup_currency {
    pub player: i32,
    pub currency: i32,
}


pub struct mvm_quick_sentry_upgrade {
    pub player: i32,
}


pub struct mvm_scout_marked_for_death {
    pub player: i32,
}


pub struct mvm_sentrybuster_detonate {
    pub player: i32,
    pub det_x: f32,
    pub det_y: f32,
    pub det_z: f32,
}


pub struct mvm_sentrybuster_killed {
    pub sentry_buster: i32,
}


pub struct mvm_sniper_headshot_currency {
    pub userid: i32,
    pub currency: i32,
}


pub struct mvm_wave_complete {
    pub advanced: bool,
}


pub struct nav_blocked {
    pub area: i32,
    pub blocked: bool,
}


pub struct num_cappers_changed {
    pub index: i32,
    pub count: i32,
}


pub struct object_deflected {
    pub userid: i32,
    pub ownerid: i32,
    pub weaponid: i32,
    pub object_entindex: i32,
}


pub struct object_destroyed {
    pub userid: i32,
    pub attacker: i32,
    pub assister: i32,
    pub weapon: String,
    pub weaponid: i32,
    pub objecttype: i32,
    pub index: i32,
    pub was_building: bool,
}


pub struct object_detonated {
    pub userid: i32,
    pub objecttype: i32,
    pub index: i32,
}


pub struct object_removed {
    pub userid: i32,
    pub objecttype: i32,
    pub index: i32,
}


pub struct parachute_deploy {
    pub index: i32,
}


pub struct parachute_holster {
    pub index: i32,
}


pub struct party_chat {
    pub steamid: String,
    pub text: String,
    pub types: i32,
}


pub struct party_member_join {
    pub steamid: String,
}


pub struct party_member_leave {
    pub steamid: String,
}


pub struct pass_ball_blocked {
    pub owner: i32,
    pub blocker: i32,
}


pub struct pass_ball_stolen {
    pub victim: i32,
    pub attacker: i32,
}


pub struct pass_free {
    pub owner: i32,
    pub attacker: i32,
}


pub struct pass_get {
    pub owner: i32,
}


pub struct pass_pass_caught {
    pub passer: i32,
    pub catcher: i32,
    pub dist: f32,
    pub duration: f32,
}


pub struct pass_score {
    pub scorer: i32,
    pub assister: i32,
    pub poi32s: i32,
}


pub struct path_track_passed {
    pub index: i32,
}


pub struct payload_pushed {
    pub pusher: i32,
    pub distance: String,
}


pub struct player_abandoned_match {
    pub game_over: bool,
}


pub struct player_account_changed {
    pub old_value: i32,
    pub new_value: i32,
}


pub struct player_askedforball {
    pub userid: i32,
}


pub struct player_bonuspoi32s {
    pub poi32s: i32,
    pub player_entindex: i32,
    pub source_entindex	: i32,
}


pub struct player_buff {
    pub userid: i32,
    pub buff_owner: i32,
    pub buff_type: i32,
}


pub struct player_builtobject {
    pub userid: i32,
    pub object: i32,
    pub index: i32,
}


pub struct player_buyback {
    pub player: i32,
    pub cost: i32,
}


pub struct player_calledformedic {
    pub userid	: i32,
}


pub struct player_carryobject {
    pub userid: i32,
    pub object: i32,
    pub index: i32,
}


pub struct player_changeclass {
    pub userid: i32,
    pub class: i32,
}


pub struct player_chargedeployed {
    pub userid: i32,
    pub targetid: i32,
}


pub struct player_currency_changed {
    pub currency: i32,
}


pub struct player_damage_dodged {
    pub damage: i32,
}


pub struct player_damaged {
    pub amount: i32,
    pub types: i32,

}


pub struct player_death {
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


pub struct player_destroyed_pipebomb {
    pub userid: i32,
}


pub struct player_directhit_stun {
    pub attacker: i32,
    pub victim: i32,
}


pub struct player_domination {
    pub dominator: i32,
    pub dominated: i32,
    pub dominations: i32,
}


pub struct player_dropobject {
    pub userid: i32,
    pub object: i32,
    pub index: i32,
}


pub struct player_escort_score {
    pub player: i32,
    pub poi32s: i32,
}


pub struct player_extinguished {
    pub victim: i32,
    pub healer: i32,
    pub itemdefindex: i32,
}


pub struct player_healed {
    pub patient: i32,
    pub healer	: i32,
    pub amount: i32,
}


pub struct player_healedbymedic {
    pub medic: i32,
}


pub struct player_healedmediccall {
    pub userid: i32,
}


pub struct player_healonhit {
    pub amount: i32,
    pub entindex: i32,
}


pub struct player_highfive_cancel {
    pub entindex: i32,
}


pub struct player_highfive_success {
    pub initiator_entindex: i32,
    pub partner_entindex	: i32,
}


pub struct player_hurt {
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


pub struct player_ignited_inv {
    pub pyro_entindex: i32,
    pub victim_entindex: i32,
    pub medic_entindex: i32,
}


pub struct player_ignited {
    pub pyro_entindex: i32,
    pub victim_entindex: i32,
    pub weaponid: i32,
}


pub struct player_initial_spawn {
    pub index: i32,
}


pub struct player_invulned {
    pub userid: i32,
    pub medic_userid: i32,
}


pub struct player_jarated_fade {
    pub thrower_entindex: i32,
    pub victim_entindex: i32,
}


pub struct player_jarated {
    pub thrower_entindex: i32,
    pub victim_entindex: i32,
}


pub struct player_killed_achievement_zone {
    pub attacker: i32,
    pub victim: i32,
    pub zone_id: i32,
}


pub struct player_mvp {
    pub player: i32,
}


pub struct player_next_map_vote_change {
    pub map_index: i32,
    pub vote: i32,
}


pub struct player_pinned {
    pub pinned: i32,
}


pub struct player_rocketpack_pushed {
    pub pusher: i32,
    pub pushed: i32,
}


pub struct player_sapped_object {
    pub userid: i32,
    pub ownerid: i32,
    pub object: i32,
    pub sapperid: i32,
}


pub struct player_score_changed {
    pub player: i32,
    pub delta: i32,
}


pub struct player_shield_blocked {
    pub attacker_entindex: i32,
    pub blocker_entindex: i32,
}


pub struct player_spawn {
    pub userid: i32,
    pub team: i32,
    pub class: i32,
}


pub struct player_stats_updated {
    pub forceupload: bool,
}


pub struct player_stealsandvich {
    pub owner: i32,
    pub target: i32,
}


pub struct player_stunned {
    pub stunner: i32,
    pub victim: i32,
    pub victim_capping	: bool,
    pub big_stun: bool,
}


pub struct player_teleported {
    pub userid: i32,
    pub builderid: i32,
    pub dist: f32,
}


pub struct player_turned_to_ghost {
    pub userid: i32,
}


pub struct player_upgradedobject {
    pub userid: i32,
    pub object: i32,
    pub index: i32,
    pub isbuilder: bool,
}


pub struct player_used_powerup_bottle {
    pub player: i32,
    pub types: i32,
    pub time: f32,
}


pub struct post_inventory_application {
    pub userid: i32,
}


pub struct projectile_direct_hit {
    pub attacker: i32,
    pub victim: i32,
    pub weapon_def_index: i32,
}


pub struct projectile_removed {
    pub attacker: i32,
    pub weapon_def_index: i32,
    pub num_hit: i32,
    pub num_direct_hit: i32,
}


pub struct proto_def_changed {
    pub types: i32,
    pub defindex: i32,
    pub created: bool,
    pub deleted: bool,
    pub erase_history: bool,
}


pub struct pve_win_panel {
    pub panel_style: i32,
    pub winning_team: i32,
    pub winreason: String,
}


pub struct quest_objective_completed {
    pub quest_item_id_low: i32,
    pub quest_item_id_hi: i32,
    pub quest_objective_id: i32,
    pub scorer_user_id: i32,
}


pub struct quest_progress {
    pub owner: i32,
    pub scorer: i32,
    pub types: i32,
    pub completed: bool,
    pub quest_defindex: i32,
}


pub struct quest_request {
    pub request: i32,
    pub msg: String,
}


pub struct quest_response {
    pub request: i32,
    pub success: bool,
    pub msg: String,
}


pub struct quest_turn_in_state {
    pub state: i32,
}


pub struct rd_robot_impact {
    pub entindex: i32,
    pub impulse_x: f32,
    pub impulse_y: f32,
    pub impulse_z: f32,
}


pub struct rd_robot_killed {
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


pub struct rd_team_poi32s_changed {
    pub poi32s: i32,
    pub team: i32,
    pub method: i32,
}


pub struct rematch_vote_period_over {
    pub success: bool,
}


pub struct remove_nemesis_relationships {
    pub player: i32,
}


pub struct respawn_ghost {
    pub reviver: i32,
    pub ghost: i32,
}


pub struct restart_timer_time {
    pub time: String,
}


pub struct revive_player_complete {
    pub entindex: i32,
}


pub struct revive_player_notify {
    pub entindex: i32,
    pub marker_entindex: i32,
}


pub struct revive_player_stopped {
    pub entindex: i32,
}


pub struct rocket_jump_landed {
    pub userid: i32,
}


pub struct rocket_jump {
    pub userid: i32,
    pub playsound: bool,
}


pub struct rocketpack_landed {
    pub userid: i32,
}


pub struct rocketpack_launch {
    pub userid: i32,
    pub playsound: bool,
}


pub struct rps_taunt_event {
    pub winner: i32,
    pub winner_rps: i32,
    pub loser: i32,
    pub loser_rps: i32,
}


pub struct scout_grand_slam {
    pub scout_id: i32,
    pub target_id: i32,
}


pub struct scout_slamdoll_landed {
    pub target_index: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


pub struct sentry_on_go_active {
    pub index: i32,
}


pub struct show_annotation {
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


pub struct show_class_layout {
    pub show: bool,
}


pub struct show_freezepanel {
    pub killer: i32,
}


pub struct show_vs_panel {
    pub show: bool,
}


pub struct special_score {
    pub player: i32,
}


pub struct sticky_jump_landed {
    pub userid: i32,
}


pub struct sticky_jump {
    pub userid: i32,
    pub playsound: bool,
}


pub struct tagged_player_as_it {
    pub player: i32,
}


pub struct team_leader_killed {
    pub killer: i32,
    pub victim: i32,
}


pub struct teamplay_alert {
    pub alert_type: i32,
}


pub struct teamplay_broadcast_audio {
    pub team: i32,
    pub sound: String,
    pub additional_flags: i32,
}


pub struct teamplay_capture_blocked {
    pub cp: i32,
    pub cpname: String,
    pub blocker: i32,
    pub victim: i32,
}


pub struct teamplay_capture_broken {
    pub cp: i32,
    pub cpname: String,
    pub time_remaining	: f32,
}


pub struct teamplay_flag_event {
    pub player: i32,
    pub carrier: i32,
    pub eventtype: i32,
    pub home: i32,
    pub team: i32,
}


pub struct teamplay_game_over {
    pub reason: String,
}


pub struct teamplay_map_time_remaining {
    pub seconds: i32,
}


pub struct teamplay_poi32_captured {
    pub cp: i32,
    pub cpname: String,
    pub team: i32,
    pub cappers: String,
}


pub struct teamplay_poi32_locked {
    pub cp: i32,
    pub cpname: String,
    pub team: i32,
}


pub struct teamplay_poi32_startcapture {
    pub cp: i32,
    pub cpname: String,
    pub team: i32,
    pub capteam: i32,
    pub cappers: String,
    pub captime: f32,
}


pub struct teamplay_poi32_unlocked {
    pub cp: i32,
    pub cpname: String,
    pub 	team: i32,
}


pub struct teamplay_pre_round_time_left {
    pub time: i32,
}


pub struct teamplay_round_restart_seconds {
    pub seconds: i32,
}


pub struct teamplay_round_selected {
    pub round: String,
}


pub struct teamplay_round_stalemate {
    pub reason: String,
}


pub struct teamplay_round_start {
    pub full_reset: bool,
}


pub struct teamplay_round_win {
    pub team: i32,
    pub winreason: i32,
    pub flagcaplimit: i32,
    pub full_round: bool,
    pub round_time: f32,
    pub losing_team_num_caps: i32,
    pub was_sudden_death: bool,
}


pub struct teamplay_team_ready {
    pub team: i32,
}


pub struct teamplay_teambalanced_player {
    pub player: i32,
    pub team: i32,
}


pub struct teamplay_timer_flash {
    pub time_remaining: i32,
}


pub struct teamplay_timer_time_added {
    pub timer: i32,
    pub seconds_added: i32,
}


pub struct teamplay_win_panel {
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


pub struct tf_game_over {
    pub reason: String,
}


pub struct tf_map_time_remaining {
    pub seconds: i32,
}


pub struct tournament_stateupdate {
    pub userid: i32,
    pub namechange: bool,
    pub readystate: i32,
    pub newname: String,
}


pub struct training_complete {
    pub next_map: String,
    pub map: String,
    pub text: String,
}


pub struct update_status_item {
    pub index: i32,
    pub object: i32,
}


pub struct upgrades_file_changed {
    pub path: String,
}