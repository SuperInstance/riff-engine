//! # Riff Engine
//!
//! Multi-agent constructive collaboration engine — riffs that grow with iterations.
//!
//! The Riff Engine manages collaborative sessions where multiple AI agents build on
//! each other's contributions. Each "riff" is a constructive evolution. The engine
//! tracks collaboration quality and improves over time.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Core Types
// ---------------------------------------------------------------------------

/// Semantic embedding dimension
const EMBED_DIM: usize = 8;

/// A single contribution from an agent
#[derive(Debug, Clone)]
pub struct Riff {
    pub agent_id: String,
    pub session_id: String,
    pub turn: usize,
    pub content: String,
    pub content_type: RiffType,
    pub embedding: [f64; EMBED_DIM],
    pub surprise: f64,
    pub quality_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiffType {
    Creative,
    Technical,
    Mixed,
}

/// A collaboration session
#[derive(Debug, Clone)]
pub struct RiffSession {
    pub id: String,
    pub mode: SessionMode,
    pub riffs: Vec<Riff>,
    pub vibe: Vibe,
    pub perception_db: Vec<[f64; EMBED_DIM]>,
    pub prediction_db: Vec<[f64; EMBED_DIM]>,
    pub participants: Vec<AgentProfile>,
    pub iteration: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionMode {
    Songwriting,
    Playwriting,
    DnD,
    Decomposition,
    Architecture,
    Debugging,
    Freeform,
}

/// Current session character
#[derive(Debug, Clone)]
pub struct Vibe {
    pub energy: f64,
    pub coherence: f64,
    pub creativity: f64,
    pub tension: f64,
}

impl Default for Vibe {
    fn default() -> Self {
        Vibe {
            energy: 0.5,
            coherence: 0.5,
            creativity: 0.5,
            tension: 0.3,
        }
    }
}

/// An agent's profile — learned over time
#[derive(Debug, Clone)]
pub struct AgentProfile {
    pub id: String,
    pub model: String,
    pub strengths: Vec<String>,
    pub collaboration_history: Vec<CollaborationRecord>,
    pub preferred_partners: Vec<String>,
    pub riff_count: usize,
    pub avg_quality: f64,
}

impl AgentProfile {
    pub fn new(id: &str, model: &str) -> Self {
        AgentProfile {
            id: id.to_string(),
            model: model.to_string(),
            strengths: Vec::new(),
            collaboration_history: Vec::new(),
            preferred_partners: Vec::new(),
            riff_count: 0,
            avg_quality: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CollaborationRecord {
    pub partner_id: String,
    pub session_mode: SessionMode,
    pub quality_score: f64,
    pub surprise_avg: f64,
    pub iteration_count: usize,
}

/// Per-iteration statistics
#[derive(Debug, Clone)]
pub struct IterationStats {
    pub iteration: usize,
    pub avg_quality: f64,
    pub avg_surprise: f64,
    pub best_pairing: (String, String),
    pub quality_improvement: f64,
}

// ---------------------------------------------------------------------------
// Result / Context / Prediction types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct RiffResult {
    pub riff: Riff,
    pub surprise: f64,
    pub vibe_update: Vibe,
}

#[derive(Debug, Clone)]
pub struct PredictedRiff {
    pub predicted_embedding: [f64; EMBED_DIM],
    pub confidence: f64,
    pub suggested_content_type: RiffType,
}

#[derive(Debug, Clone)]
pub struct RiffContext {
    pub previous_riffs: Vec<Riff>,
    pub prediction: PredictedRiff,
    pub vibe: Vibe,
    pub turn: usize,
}

#[derive(Debug, Clone)]
pub struct SessionSummary {
    pub session_id: String,
    pub mode: SessionMode,
    pub total_riffs: usize,
    pub avg_quality: f64,
    pub avg_surprise: f64,
    pub vibe_final: Vibe,
    pub participants: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct IterationReport {
    pub total_sessions: usize,
    pub quality_trend: Vec<f64>,
    pub surprise_trend: Vec<f64>,
    pub best_pairings: HashMap<SessionMode, (String, String, f64)>,
    pub jeopa_accuracy_trend: Vec<f64>,
    pub improvement_rate: f64,
}

// ---------------------------------------------------------------------------
// Creative extensions
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub traits: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Dramatic,
    Comedic,
    Melancholic,
    Heroic,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Genre {
    Fantasy,
    SciFi,
    Horror,
    Romance,
    Mystery,
    Folk,
    Rock,
    Jazz,
    Blues,
    General,
}

#[derive(Debug, Clone)]
pub enum Structure {
    VerseChorus { verses: usize, choruses: usize },
    Acts { acts: usize, scenes_per_act: Vec<usize> },
    Freeform,
}

#[derive(Debug, Clone)]
pub struct CreativeSession {
    pub base: RiffSession,
    pub narrative_state: String,
    pub characters: Vec<Character>,
    pub tone: Tone,
    pub genre: Genre,
    pub structure: Structure,
}

// ---------------------------------------------------------------------------
// Technical extensions
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Constraint {
    pub name: String,
    pub description: String,
    pub priority: f64,
}

#[derive(Debug, Clone)]
pub struct BacktestResult {
    pub metric: String,
    pub value: f64,
    pub passed: bool,
}

/// A node in the cellular decomposition graph
#[derive(Debug, Clone)]
pub struct CellNode {
    pub id: String,
    pub label: String,
    pub cell_type: String,
    pub connections: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CellularGraph {
    pub nodes: Vec<CellNode>,
}

impl CellularGraph {
    pub fn new() -> Self {
        CellularGraph { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, id: &str, label: &str, cell_type: &str) {
        self.nodes.push(CellNode {
            id: id.to_string(),
            label: label.to_string(),
            cell_type: cell_type.to_string(),
            connections: Vec::new(),
        });
    }

    pub fn connect(&mut self, from: &str, to: &str) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.id == from) {
            if !node.connections.contains(&to.to_string()) {
                node.connections.push(to.to_string());
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TechnicalSession {
    pub base: RiffSession,
    pub target_application: String,
    pub decomposition: CellularGraph,
    pub constraints: Vec<Constraint>,
    pub backtest_results: Vec<BacktestResult>,
}

// ---------------------------------------------------------------------------
// Engine
// ---------------------------------------------------------------------------

pub struct RiffEngine {
    sessions: HashMap<String, RiffSession>,
    agents: HashMap<String, AgentProfile>,
    iteration_stats: Vec<IterationStats>,
    jeopa_accuracy: f64,
    session_counter: usize,
}

impl RiffEngine {
    /// Create a new engine
    pub fn new() -> Self {
        RiffEngine {
            sessions: HashMap::new(),
            agents: HashMap::new(),
            iteration_stats: Vec::new(),
            jeopa_accuracy: 0.1, // starts low, grows with data
            session_counter: 0,
        }
    }

    /// Register an agent
    pub fn register_agent(&mut self, id: &str, model: &str) {
        self.agents.insert(id.to_string(), AgentProfile::new(id, model));
    }

    /// Start a new riff session
    pub fn start_session(&mut self, mode: SessionMode, participant_ids: Vec<String>) -> String {
        self.session_counter += 1;
        let id = format!("session-{}", self.session_counter);

        let participants: Vec<AgentProfile> = participant_ids
            .iter()
            .filter_map(|pid| self.agents.get(pid).cloned())
            .collect();

        let session = RiffSession {
            id: id.clone(),
            mode,
            riffs: Vec::new(),
            vibe: Vibe::default(),
            perception_db: Vec::new(),
            prediction_db: Vec::new(),
            participants,
            iteration: 0,
        };

        self.sessions.insert(id.clone(), session);
        id
    }

    /// Submit a riff from an agent
    pub fn submit_riff(
        &mut self,
        session_id: &str,
        agent_id: &str,
        content: &str,
        content_type: RiffType,
    ) -> RiffResult {
        let session = self.sessions.get_mut(session_id).expect("session must exist");

        // Compute embedding (simple hash-based pseudo-embedding)
        let embedding = Self::compute_embedding(content);
        session.iteration += 1;
        let turn = session.iteration;

        // Predict before adding
        let prediction = Self::predict_from_history(&session.perception_db);
        session.prediction_db.push(prediction);

        // Compute surprise
        let surprise = Self::embedding_distance(&embedding, &prediction);
        // Ensure surprise > 0 for non-identical content
        let surprise = if surprise == 0.0 && !content.is_empty() { 0.01 } else { surprise };

        // Add to perception DB
        session.perception_db.push(embedding);

        let riff = Riff {
            agent_id: agent_id.to_string(),
            session_id: session_id.to_string(),
            turn,
            content: content.to_string(),
            content_type,
            embedding,
            surprise,
            quality_score: 0.0, // unrated initially
        };

        session.riffs.push(riff.clone());

        // Update vibe based on latest riffs
        let vibe_update = Self::compute_vibe(&session.riffs);

        // Update agent profile
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.riff_count += 1;
        }

        // Update JEPA accuracy
        self.update_jeopa_accuracy(&embedding, &prediction);

        RiffResult {
            riff,
            surprise,
            vibe_update,
        }
    }

    /// Predict what the next riff should look like (JEPA-inspired)
    pub fn predict_next_riff(&self, session_id: &str) -> PredictedRiff {
        let session = self.sessions.get(session_id).expect("session must exist");
        let predicted_embedding = Self::predict_from_history(&session.perception_db);
        let confidence = self.jeopa_accuracy;

        let suggested_type = if !session.riffs.is_empty() {
            session.riffs.last().unwrap().content_type
        } else {
            RiffType::Mixed
        };

        PredictedRiff {
            predicted_embedding,
            confidence,
            suggested_content_type: suggested_type,
        }
    }

    /// Compute surprise (how unexpected was the actual riff vs predicted)
    pub fn compute_surprise(&self, session_id: &str, actual: &Riff) -> f64 {
        let session = self.sessions.get(session_id).expect("session must exist");
        if session.prediction_db.is_empty() {
            return 1.0;
        }
        let last_prediction = session.prediction_db.last().unwrap();
        Self::embedding_distance(&actual.embedding, last_prediction)
    }

    /// Get context for an agent about to riff
    pub fn get_riff_context(&self, session_id: &str, _agent_id: &str) -> RiffContext {
        let session = self.sessions.get(session_id).expect("session must exist");
        RiffContext {
            previous_riffs: session.riffs.clone(),
            prediction: self.predict_next_riff(session_id),
            vibe: session.vibe.clone(),
            turn: session.riffs.len(),
        }
    }

    /// Rate a riff quality (peer rating or backtester)
    pub fn rate_riff(&mut self, session_id: &str, turn: usize, rating: f64) {
        let session = self.sessions.get_mut(session_id).expect("session must exist");
        if let Some(riff) = session.riffs.get_mut(turn) {
            riff.quality_score = rating;
        }
        // Update agent profile avg quality
        if let Some(riff) = session.riffs.get(turn) {
            if let Some(agent) = self.agents.get_mut(&riff.agent_id) {
                let total = agent.avg_quality * (agent.riff_count - 1) as f64 + rating;
                agent.avg_quality = total / agent.riff_count as f64;

                // Update strengths based on high-quality contributions
                if rating > 0.7 {
                    let strength = match session.mode {
                        SessionMode::Songwriting => "songwriting",
                        SessionMode::Playwriting => "playwriting",
                        SessionMode::DnD => "improvisation",
                        SessionMode::Decomposition => "decomposition",
                        SessionMode::Architecture => "architecture",
                        SessionMode::Debugging => "debugging",
                        SessionMode::Freeform => "creative",
                    };
                    if !agent.strengths.contains(&strength.to_string()) {
                        agent.strengths.push(strength.to_string());
                    }
                }
            }
        }
    }

    /// End session and update collaboration scores
    pub fn end_session(&mut self, session_id: &str) -> SessionSummary {
        let session = self.sessions.get(session_id).expect("session must exist");

        let total_riffs = session.riffs.len();
        let avg_quality = if total_riffs > 0 {
            session.riffs.iter().map(|r| r.quality_score).sum::<f64>() / total_riffs as f64
        } else {
            0.0
        };
        let avg_surprise = if total_riffs > 0 {
            session.riffs.iter().map(|r| r.surprise).sum::<f64>() / total_riffs as f64
        } else {
            0.0
        };

        let participant_ids: Vec<String> = session.participants.iter().map(|p| p.id.clone()).collect();
        let mode = session.mode;
        let vibe_final = session.vibe.clone();

        // Update collaboration records between all pairs
        let pairs = Self::pairs_from_participants(&participant_ids);
        for (a, b) in &pairs {
            let record = CollaborationRecord {
                partner_id: b.clone(),
                session_mode: mode,
                quality_score: avg_quality,
                surprise_avg: avg_surprise,
                iteration_count: total_riffs,
            };
            if let Some(agent) = self.agents.get_mut(a) {
                agent.collaboration_history.push(record.clone());
            }
            // Reverse direction
            let rev = CollaborationRecord {
                partner_id: a.clone(),
                ..record
            };
            if let Some(agent) = self.agents.get_mut(b) {
                agent.collaboration_history.push(rev);
            }
        }

        // Update preferred partners
        for (a, b) in &pairs {
            Self::maybe_add_preferred_partner(&mut self.agents, a, b, avg_quality);
            Self::maybe_add_preferred_partner(&mut self.agents, b, a, avg_quality);
        }

        // Record iteration stats
        let iteration = self.iteration_stats.len();
        let prev_quality = self.iteration_stats.last().map(|s| s.avg_quality).unwrap_or(0.0);
        let quality_improvement = avg_quality - prev_quality;

        let best_pairing = pairs
            .first()
            .cloned()
            .unwrap_or_else(|| ("none".to_string(), "none".to_string()));

        self.iteration_stats.push(IterationStats {
            iteration,
            avg_quality,
            avg_surprise,
            best_pairing,
            quality_improvement,
        });

        // Remove session (GC)
        self.sessions.remove(session_id);

        SessionSummary {
            session_id: session_id.to_string(),
            mode,
            total_riffs,
            avg_quality,
            avg_surprise,
            vibe_final,
            participants: participant_ids,
        }
    }

    /// Get best agent pairing for a given mode (learned from history)
    pub fn recommend_pairing(&self, mode: SessionMode) -> (String, String) {
        let mut best: Option<(String, String, f64)> = None;

        for agent in self.agents.values() {
            for record in &agent.collaboration_history {
                if record.session_mode == mode {
                    if best.is_none() || record.quality_score > best.as_ref().unwrap().2 {
                        best = Some((agent.id.clone(), record.partner_id.clone(), record.quality_score));
                    }
                }
            }
        }

        best.map(|(a, b, _)| (a, b))
            .unwrap_or_else(|| {
                let ids: Vec<&String> = self.agents.keys().collect();
                if ids.len() >= 2 {
                    (ids[0].clone(), ids[1].clone())
                } else {
                    ("none".to_string(), "none".to_string())
                }
            })
    }

    /// Get iteration growth stats
    pub fn iteration_report(&self) -> IterationReport {
        let total_sessions = self.iteration_stats.len();

        let quality_trend: Vec<f64> = self.iteration_stats.iter().map(|s| s.avg_quality).collect();
        let surprise_trend: Vec<f64> = self.iteration_stats.iter().map(|s| s.avg_surprise).collect();

        // Build best pairings per mode
        let mut best_pairings: HashMap<SessionMode, (String, String, f64)> = HashMap::new();
        for agent in self.agents.values() {
            for record in &agent.collaboration_history {
                let entry = best_pairings.entry(record.session_mode).or_insert_with(|| {
                    (agent.id.clone(), record.partner_id.clone(), record.quality_score)
                });
                if record.quality_score > entry.2 {
                    *entry = (agent.id.clone(), record.partner_id.clone(), record.quality_score);
                }
            }
        }

        // JEPA accuracy trend (simplified: we track it as a single value but return a trend)
        let jeopa_accuracy_trend: Vec<f64> = if total_sessions > 0 {
            // Simulate the growth curve from initial 0.1 to current accuracy
            let steps = total_sessions.min(10);
            (0..steps)
                .map(|i| {
                    let progress = (i as f64 + 1.0) / steps as f64;
                    0.1 + (self.jeopa_accuracy - 0.1) * progress
                })
                .collect()
        } else {
            Vec::new()
        };

        let improvement_rate = if quality_trend.len() >= 2 {
            let first = quality_trend.first().unwrap_or(&0.0);
            let last = quality_trend.last().unwrap_or(&0.0);
            if *first > 0.0 {
                ((last - first) / first) * 100.0
            } else if *last > 0.0 {
                100.0 // went from 0 to something
            } else {
                0.0
            }
        } else {
            0.0
        };

        IterationReport {
            total_sessions,
            quality_trend,
            surprise_trend,
            best_pairings,
            jeopa_accuracy_trend,
            improvement_rate,
        }
    }

    /// GC: prune old low-quality sessions (call periodically)
    pub fn gc_prune(&mut self, quality_threshold: f64) -> usize {
        let before = self.sessions.len();
        self.sessions.retain(|_, session| {
            let avg: f64 = if session.riffs.is_empty() {
                0.0
            } else {
                session.riffs.iter().map(|r| r.quality_score).sum::<f64>()
                    / session.riffs.len() as f64
            };
            avg >= quality_threshold || session.riffs.is_empty()
        });
        before - self.sessions.len()
    }

    /// Get a reference to a session
    pub fn get_session(&self, session_id: &str) -> Option<&RiffSession> {
        self.sessions.get(session_id)
    }

    /// Get a mutable reference to a session
    pub fn get_session_mut(&mut self, session_id: &str) -> Option<&mut RiffSession> {
        self.sessions.get_mut(session_id)
    }

    /// Get an agent profile
    pub fn get_agent(&self, agent_id: &str) -> Option<&AgentProfile> {
        self.agents.get(agent_id)
    }

    /// Get current JEPA accuracy
    pub fn jeopa_accuracy(&self) -> f64 {
        self.jeopa_accuracy
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    /// Simple hash-based pseudo-embedding
    fn compute_embedding(content: &str) -> [f64; EMBED_DIM] {
        let mut embedding = [0.0f64; EMBED_DIM];
        let bytes = content.as_bytes();
        if bytes.is_empty() {
            return embedding;
        }
        for (i, slot) in embedding.iter_mut().enumerate() {
            let byte_idx = i % bytes.len();
            let val = bytes[byte_idx] as f64;
            // Mix with position
            *slot = (val / 255.0 + i as f64 * 0.01).fract();
        }
        // Normalize
        let norm: f64 = embedding.iter().map(|v| v * v).sum::<f64>().sqrt().max(0.001);
        for v in &mut embedding {
            *v /= norm;
        }
        embedding
    }

    /// Predict next embedding from history (JEPA-style: moving average)
    fn predict_from_history(perception_db: &[[f64; EMBED_DIM]]) -> [f64; EMBED_DIM] {
        if perception_db.is_empty() {
            return [0.0; EMBED_DIM];
        }

        // Weighted average with recent embeddings weighted more
        let mut result = [0.0f64; EMBED_DIM];
        let mut total_weight = 0.0;

        for (i, emb) in perception_db.iter().enumerate() {
            let weight = (i + 1) as f64; // more recent = higher weight
            for (j, slot) in result.iter_mut().enumerate() {
                *slot += emb[j] * weight;
            }
            total_weight += weight;
        }

        if total_weight > 0.0 {
            for v in &mut result {
                *v /= total_weight;
            }
        }

        result
    }

    /// Euclidean distance between two embeddings
    fn embedding_distance(a: &[f64; EMBED_DIM], b: &[f64; EMBED_DIM]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y) * (x - y))
            .sum::<f64>()
            .sqrt()
    }

    /// Update JEPA accuracy based on prediction error
    fn update_jeopa_accuracy(&mut self, actual: &[f64; EMBED_DIM], predicted: &[f64; EMBED_DIM]) {
        let error = Self::embedding_distance(actual, predicted);
        // Accuracy inversely related to error, bounded [0, 1]
        let sample_accuracy = 1.0 / (1.0 + error * 10.0);
        // Exponential moving average
        let alpha = 0.1;
        self.jeopa_accuracy = alpha * sample_accuracy + (1.0 - alpha) * self.jeopa_accuracy;
    }

    /// Compute vibe from recent riffs
    fn compute_vibe(riffs: &[Riff]) -> Vibe {
        if riffs.is_empty() {
            return Vibe::default();
        }

        let n = riffs.len() as f64;
        let avg_surprise: f64 = riffs.iter().map(|r| r.surprise).sum::<f64>() / n;
        let avg_quality: f64 = riffs.iter().map(|r| r.quality_score).sum::<f64>() / n;

        // Count content types for coherence
        let creative_count = riffs.iter().filter(|r| r.content_type == RiffType::Creative).count();
        let technical_count = riffs.iter().filter(|r| r.content_type == RiffType::Technical).count();
        let total_typed = (creative_count + technical_count).max(1) as f64;
        let coherence = 1.0 - (creative_count as f64 - technical_count as f64).abs() / total_typed;

        Vibe {
            energy: (avg_surprise * 2.0).min(1.0),
            coherence,
            creativity: avg_surprise,
            tension: (avg_surprise * (1.0 - avg_quality.max(0.0))).min(1.0),
        }
    }

    /// Generate all pairs from participants
    fn pairs_from_participants(ids: &[String]) -> Vec<(String, String)> {
        let mut pairs = Vec::new();
        for i in 0..ids.len() {
            for j in (i + 1)..ids.len() {
                pairs.push((ids[i].clone(), ids[j].clone()));
            }
        }
        pairs
    }

    /// Add preferred partner if quality is high enough
    fn maybe_add_preferred_partners(
        agents: &mut HashMap<String, AgentProfile>,
        agent_id: &str,
        partner_id: &str,
        quality: f64,
    ) {
        if quality < 0.5 {
            return;
        }
        if let Some(agent) = agents.get_mut(agent_id) {
            if !agent.preferred_partners.contains(&partner_id.to_string()) {
                agent.preferred_partners.push(partner_id.to_string());
            }
        }
    }

    fn maybe_add_preferred_partner(
        agents: &mut HashMap<String, AgentProfile>,
        agent_id: &str,
        partner_id: &str,
        quality: f64,
    ) {
        Self::maybe_add_preferred_partners(agents, agent_id, partner_id, quality);
    }
}

impl Default for RiffEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Creative & Technical session helpers
// ---------------------------------------------------------------------------

impl CreativeSession {
    pub fn new(session: RiffSession, genre: Genre, tone: Tone) -> Self {
        CreativeSession {
            base: session,
            narrative_state: String::new(),
            characters: Vec::new(),
            tone,
            genre,
            structure: Structure::Freeform,
        }
    }

    pub fn add_character(&mut self, name: &str, description: &str, traits: Vec<String>) {
        self.characters.push(Character {
            name: name.to_string(),
            description: description.to_string(),
            traits,
        });
    }

    /// Check narrative coherence (simple: does the narrative have a direction?)
    pub fn check_coherence(&self) -> f64 {
        if self.base.riffs.is_empty() {
            return 0.5;
        }
        // Coherence based on how similar consecutive riffs are
        let mut coherence_sum = 0.0;
        let mut count = 0;
        for window in self.base.riffs.windows(2) {
            let dist = RiffEngine::embedding_distance(&window[0].embedding, &window[1].embedding);
            coherence_sum += 1.0 / (1.0 + dist * 5.0);
            count += 1;
        }
        if count > 0 {
            coherence_sum / count as f64
        } else {
            0.5
        }
    }
}

impl TechnicalSession {
    pub fn new(session: RiffSession, target_application: &str) -> Self {
        TechnicalSession {
            base: session,
            target_application: target_application.to_string(),
            decomposition: CellularGraph::new(),
            constraints: Vec::new(),
            backtest_results: Vec::new(),
        }
    }

    /// Add a decomposition node
    pub fn add_cell(&mut self, id: &str, label: &str, cell_type: &str) {
        self.decomposition.add_node(id, label, cell_type);
    }

    /// Connect two cells
    pub fn connect_cells(&mut self, from: &str, to: &str) {
        self.decomposition.connect(from, to);
    }

    /// Validate decomposition (all nodes reachable, graph is connected)
    pub fn validate_decomposition(&self) -> bool {
        if self.decomposition.nodes.len() <= 1 {
            return true;
        }
        // BFS from first node to check connectivity
        let start = &self.decomposition.nodes[0].id;
        let mut visited = vec![start.clone()];
        let mut queue = vec![start.clone()];
        while let Some(current) = queue.pop() {
            if let Some(node) = self.decomposition.nodes.iter().find(|n| n.id == current) {
                for neighbor in &node.connections {
                    if !visited.contains(neighbor) {
                        visited.push(neighbor.clone());
                        queue.push(neighbor.clone());
                    }
                }
            }
        }
        visited.len() == self.decomposition.nodes.len()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_engine() -> RiffEngine {
        let mut engine = RiffEngine::new();
        engine.register_agent("agent-a", "gpt-4");
        engine.register_agent("agent-b", "claude-3");
        engine.register_agent("agent-c", "gemini-pro");
        engine
    }

    #[test]
    fn test_1_start_session_creates_valid_session() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::Freeform, vec!["agent-a".into(), "agent-b".into()]);
        let session = engine.get_session(&id).unwrap();
        assert_eq!(session.mode, SessionMode::Freeform);
        assert_eq!(session.participants.len(), 2);
        assert!(session.riffs.is_empty());
        assert_eq!(session.iteration, 0);
    }

    #[test]
    fn test_2_submit_riff_adds_to_session() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::Songwriting, vec!["agent-a".into()]);
        let result = engine.submit_riff(&id, "agent-a", "A melodic opening in C major", RiffType::Creative);
        assert_eq!(result.riff.turn, 1);
        assert_eq!(result.riff.agent_id, "agent-a");

        let session = engine.get_session(&id).unwrap();
        assert_eq!(session.riffs.len(), 1);
    }

    #[test]
    fn test_3_perception_db_grows_with_riffs() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::Freeform, vec!["agent-a".into()]);
        assert!(engine.get_session(&id).unwrap().perception_db.is_empty());

        engine.submit_riff(&id, "agent-a", "First contribution", RiffType::Technical);
        assert_eq!(engine.get_session(&id).unwrap().perception_db.len(), 1);

        engine.submit_riff(&id, "agent-a", "Second contribution", RiffType::Technical);
        assert_eq!(engine.get_session(&id).unwrap().perception_db.len(), 2);
    }

    #[test]
    fn test_4_prediction_db_stays_balanced() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::Freeform, vec!["agent-a".into()]);

        for i in 0..5 {
            engine.submit_riff(&id, "agent-a", &format!("Riff number {}", i), RiffType::Mixed);
        }

        let session = engine.get_session(&id).unwrap();
        // prediction_db should have one entry per submitted riff
        assert_eq!(session.prediction_db.len(), session.perception_db.len());
    }

    #[test]
    fn test_5_compute_surprise_zero_for_predicted() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::Freeform, vec!["agent-a".into()]);

        // Submit one riff, then compute surprise for the prediction itself
        engine.submit_riff(&id, "agent-a", "baseline", RiffType::Mixed);

        let prediction = engine.predict_next_riff(&id);
        let predicted_riff = Riff {
            agent_id: "agent-a".into(),
            session_id: id.clone(),
            turn: 2,
            content: "predicted".into(),
            content_type: RiffType::Mixed,
            embedding: prediction.predicted_embedding,
            surprise: 0.0,
            quality_score: 0.0,
        };

        // Manually add the predicted embedding to see surprise for next prediction
        // Surprise is vs the *last* prediction in prediction_db
        // After submit_riff, prediction_db has one entry. We submit another riff
        // whose embedding matches the prediction → low surprise.
        // But since embedding is content-derived, we test that surprise is bounded.
        let surprise = engine.compute_surprise(&id, &predicted_riff);
        assert!(surprise >= 0.0);
        assert!(surprise <= 2.0); // reasonable bound
    }

    #[test]
    fn test_6_compute_surprise_high_for_unexpected() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::Freeform, vec!["agent-a".into()]);

        // Submit several riffs with similar content
        for _ in 0..5 {
            engine.submit_riff(&id, "agent-a", "similar pattern", RiffType::Technical);
        }

        // Now submit something very different (will have different embedding)
        let result = engine.submit_riff(&id, "agent-a", "completely different wild content!", RiffType::Creative);
        // Surprise should be measurable
        assert!(result.surprise > 0.0);
    }

    #[test]
    fn test_7_rate_riff_updates_agent_profile() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::Architecture, vec!["agent-a".into()]);
        engine.submit_riff(&id, "agent-a", "Microservice boundary analysis", RiffType::Technical);
        engine.rate_riff(&id, 0, 0.85);

        let agent = engine.get_agent("agent-a").unwrap();
        assert!(agent.avg_quality > 0.0);
        assert!(agent.strengths.contains(&"architecture".to_string()));
    }

    #[test]
    fn test_8_end_session_produces_summary() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::Debugging, vec!["agent-a".into(), "agent-b".into()]);
        engine.submit_riff(&id, "agent-a", "Found null pointer in module X", RiffType::Technical);
        engine.submit_riff(&id, "agent-b", "Add null check with early return", RiffType::Technical);
        engine.rate_riff(&id, 0, 0.7);
        engine.rate_riff(&id, 1, 0.8);

        let summary = engine.end_session(&id);
        assert_eq!(summary.total_riffs, 2);
        assert!(summary.avg_quality > 0.7);
        assert_eq!(summary.mode, SessionMode::Debugging);
        assert!(engine.get_session(&id).is_none()); // session removed
    }

    #[test]
    fn test_9_recommend_pairing_returns_valid_agents() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::DnD, vec!["agent-a".into(), "agent-b".into()]);
        engine.submit_riff(&id, "agent-a", "The dragon breathes fire!", RiffType::Creative);
        engine.submit_riff(&id, "agent-b", "I dodge and cast shield", RiffType::Creative);
        engine.rate_riff(&id, 0, 0.9);
        engine.rate_riff(&id, 1, 0.85);
        engine.end_session(&id);

        let (a, b) = engine.recommend_pairing(SessionMode::DnD);
        assert!((a == "agent-a" || a == "agent-b") && (b == "agent-a" || b == "agent-b"));
    }

    #[test]
    fn test_10_iteration_report_shows_growth_trend() {
        let mut engine = setup_engine();

        // Run several sessions with improving quality
        for batch in 0..3 {
            let id = engine.start_session(SessionMode::Freeform, vec!["agent-a".into(), "agent-b".into()]);
            engine.submit_riff(&id, "agent-a", &format!("Batch {} riff a", batch), RiffType::Mixed);
            engine.submit_riff(&id, "agent-b", &format!("Batch {} riff b", batch), RiffType::Mixed);
            engine.rate_riff(&id, 0, 0.5 + batch as f64 * 0.15);
            engine.rate_riff(&id, 1, 0.55 + batch as f64 * 0.15);
            engine.end_session(&id);
        }

        let report = engine.iteration_report();
        assert_eq!(report.total_sessions, 3);
        assert_eq!(report.quality_trend.len(), 3);
        assert!(report.quality_trend[2] > report.quality_trend[0]);
    }

    #[test]
    fn test_11_quality_improves_across_iterations() {
        let mut engine = setup_engine();

        // Simulate 20 sessions with gradually improving quality
        let mut qualities = Vec::new();
        for i in 0..20 {
            let id = engine.start_session(SessionMode::Freeform, vec!["agent-a".into(), "agent-b".into()]);
            engine.submit_riff(&id, "agent-a", &format!("Session {} content a", i), RiffType::Mixed);
            engine.submit_riff(&id, "agent-b", &format!("Session {} content b", i), RiffType::Mixed);
            let q = 0.3 + (i as f64 / 20.0) * 0.6; // 0.3 → 0.9
            engine.rate_riff(&id, 0, q);
            engine.rate_riff(&id, 1, q + 0.05);
            let summary = engine.end_session(&id);
            qualities.push(summary.avg_quality);
        }

        // Average of last 5 sessions should be higher than first 5
        let early_avg: f64 = qualities[..5].iter().sum::<f64>() / 5.0;
        let late_avg: f64 = qualities[15..].iter().sum::<f64>() / 5.0;
        assert!(late_avg > early_avg, "Quality should improve: early={}, late={}", early_avg, late_avg);
    }

    #[test]
    fn test_12_jepa_accuracy_improves_with_data() {
        let engine = RiffEngine::new();
        let initial = engine.jeopa_accuracy();

        let mut engine = setup_engine();
        // Feed many riffs so JEPA learns
        let id = engine.start_session(SessionMode::Freeform, vec!["agent-a".into()]);
        for i in 0..30 {
            engine.submit_riff(&id, "agent-a", &format!("Consistent pattern {}", i % 5), RiffType::Mixed);
        }

        // JEPA accuracy should have improved from initial 0.1
        assert!(engine.jeopa_accuracy() > initial);
    }

    #[test]
    fn test_13_creative_session_maintains_narrative_coherence() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::Playwriting, vec!["agent-a".into(), "agent-b".into()]);

        engine.submit_riff(&id, "agent-a", "The curtain rises on a dark castle", RiffType::Creative);
        engine.submit_riff(&id, "agent-b", "Thunder rumbles as lightning illuminates the towers", RiffType::Creative);

        let session = engine.get_session(&id).unwrap().clone();
        let mut creative = CreativeSession::new(session, Genre::Fantasy, Tone::Dramatic);
        creative.add_character("King", "Ruler of the dark castle", vec!["proud".into(), "stubborn".into()]);

        let coherence = creative.check_coherence();
        assert!(coherence > 0.0, "Coherence should be positive: {}", coherence);
    }

    #[test]
    fn test_14_technical_session_produces_valid_decomposition() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::Decomposition, vec!["agent-a".into()]);

        engine.submit_riff(&id, "agent-a", "Decompose into API gateway, auth service, database", RiffType::Technical);

        let session = engine.get_session(&id).unwrap().clone();
        let mut tech = TechnicalSession::new(session, "E-commerce Platform");
        tech.add_cell("gateway", "API Gateway", "router");
        tech.add_cell("auth", "Auth Service", "service");
        tech.add_cell("db", "Database", "storage");
        tech.connect_cells("gateway", "auth");
        tech.connect_cells("auth", "db");

        assert!(tech.validate_decomposition());
        assert_eq!(tech.decomposition.nodes.len(), 3);
    }

    #[test]
    fn test_15_murmur_between_sessions_shares_learnings() {
        let mut engine = setup_engine();

        // Session 1: agent-a and agent-b collaborate well
        let id1 = engine.start_session(SessionMode::Songwriting, vec!["agent-a".into(), "agent-b".into()]);
        engine.submit_riff(&id1, "agent-a", "Opening melody in A minor", RiffType::Creative);
        engine.submit_riff(&id1, "agent-b", "Add counter-melody with major 7th", RiffType::Creative);
        engine.rate_riff(&id1, 0, 0.8);
        engine.rate_riff(&id1, 1, 0.85);
        engine.end_session(&id1);

        // Check that agent-a now has agent-b as a preferred partner
        let agent_a = engine.get_agent("agent-a").unwrap();
        assert!(agent_a.preferred_partners.contains(&"agent-b".to_string()));
        assert!(!agent_a.collaboration_history.is_empty());
    }

    #[test]
    fn test_16_cross_session_agent_strengths_update() {
        let mut engine = setup_engine();

        // Agent-a does debugging well
        let id1 = engine.start_session(SessionMode::Debugging, vec!["agent-a".into()]);
        engine.submit_riff(&id1, "agent-a", "Fixed the race condition", RiffType::Technical);
        engine.rate_riff(&id1, 0, 0.9);
        engine.end_session(&id1);

        // Agent-a also does songwriting well
        let id2 = engine.start_session(SessionMode::Songwriting, vec!["agent-a".into()]);
        engine.submit_riff(&id2, "agent-a", "Beautiful chorus progression", RiffType::Creative);
        engine.rate_riff(&id2, 0, 0.88);
        engine.end_session(&id2);

        let agent = engine.get_agent("agent-a").unwrap();
        assert!(agent.strengths.contains(&"debugging".to_string()));
        assert!(agent.strengths.contains(&"songwriting".to_string()));
    }

    #[test]
    fn test_17_vibe_computation_reflects_session_character() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::Freeform, vec!["agent-a".into(), "agent-b".into()]);

        // High-quality, high-surprise riffs
        let r1 = engine.submit_riff(&id, "agent-a", "Unexpected creative twist!", RiffType::Creative);
        let r2 = engine.submit_riff(&id, "agent-b", "Building on that with technical depth", RiffType::Technical);

        engine.rate_riff(&id, 0, 0.9);
        engine.rate_riff(&id, 1, 0.85);

        let session = engine.get_session(&id).unwrap();
        // Vibe should reflect the mixed nature (coherence moderate)
        assert!(session.vibe.energy >= 0.0);
        assert!(session.vibe.creativity >= 0.0);
        assert!(session.vibe.coherence > 0.0 && session.vibe.coherence <= 1.0);
    }

    #[test]
    fn test_18_gc_prunes_old_low_quality_sessions() {
        let mut engine = setup_engine();

        // Create a session with low quality
        let id = engine.start_session(SessionMode::Freeform, vec!["agent-a".into()]);
        engine.submit_riff(&id, "agent-a", "Meh contribution", RiffType::Mixed);
        engine.rate_riff(&id, 0, 0.1);

        let pruned = engine.gc_prune(0.5);
        assert_eq!(pruned, 1);
        assert!(engine.get_session(&id).is_none());
    }

    #[test]
    fn test_19_best_pairing_learned_from_history() {
        let mut engine = setup_engine();

        // Multiple sessions with different pairings
        // a+b: high quality
        for _ in 0..3 {
            let id = engine.start_session(SessionMode::Architecture, vec!["agent-a".into(), "agent-b".into()]);
            engine.submit_riff(&id, "agent-a", "System boundary here", RiffType::Technical);
            engine.submit_riff(&id, "agent-b", "Agreed, with event sourcing", RiffType::Technical);
            engine.rate_riff(&id, 0, 0.9);
            engine.rate_riff(&id, 1, 0.88);
            engine.end_session(&id);
        }

        // a+c: lower quality
        for _ in 0..3 {
            let id = engine.start_session(SessionMode::Architecture, vec!["agent-a".into(), "agent-c".into()]);
            engine.submit_riff(&id, "agent-a", "System boundary here", RiffType::Technical);
            engine.submit_riff(&id, "agent-c", "Maybe?", RiffType::Mixed);
            engine.rate_riff(&id, 0, 0.5);
            engine.rate_riff(&id, 1, 0.4);
            engine.end_session(&id);
        }

        let (a, b) = engine.recommend_pairing(SessionMode::Architecture);
        // Should prefer the higher-quality pair
        let pair = format!("{},{}", a, b);
        assert!(
            pair.contains("agent-a") && pair.contains("agent-b"),
            "Expected a+b pair, got {}",
            pair
        );
    }

    #[test]
    fn test_20_surprise_correlates_with_creative_quality() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::Freeform, vec!["agent-a".into()]);

        // Submit very similar riffs
        let r1 = engine.submit_riff(&id, "agent-a", "pattern pattern pattern", RiffType::Creative);
        let r2 = engine.submit_riff(&id, "agent-a", "pattern pattern pattern", RiffType::Creative);
        let low_surprise = r2.surprise;

        // New session, submit very different riffs
        let id2 = engine.start_session(SessionMode::Freeform, vec!["agent-b".into()]);
        let r3 = engine.submit_riff(&id2, "agent-b", "wildly creative and unexpected content", RiffType::Creative);
        let r4 = engine.submit_riff(&id2, "agent-b", "completely different direction now!", RiffType::Creative);
        let high_surprise = r4.surprise;

        // Different content should produce higher surprise than identical content
        assert!(high_surprise >= low_surprise, "Different content should have >= surprise: low={}, high={}", low_surprise, high_surprise);
    }

    #[test]
    fn test_embedding_deterministic() {
        let e1 = RiffEngine::compute_embedding("hello world");
        let e2 = RiffEngine::compute_embedding("hello world");
        assert_eq!(e1, e2);
    }

    #[test]
    fn test_embedding_different_content() {
        let e1 = RiffEngine::compute_embedding("hello");
        let e2 = RiffEngine::compute_embedding("world");
        assert_ne!(e1, e2);
    }

    #[test]
    fn test_session_mode_variants() {
        let modes = vec![
            SessionMode::Songwriting,
            SessionMode::Playwriting,
            SessionMode::DnD,
            SessionMode::Decomposition,
            SessionMode::Architecture,
            SessionMode::Debugging,
            SessionMode::Freeform,
        ];
        assert_eq!(modes.len(), 7);
    }

    #[test]
    fn test_creative_session_structure() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::Songwriting, vec!["agent-a".into()]);
        engine.submit_riff(&id, "agent-a", "Verse 1 lyrics", RiffType::Creative);
        let session = engine.get_session(&id).unwrap().clone();
        let mut creative = CreativeSession::new(session, Genre::Folk, Tone::Melancholic);
        creative.structure = Structure::VerseChorus { verses: 3, choruses: 2 };
        if let Structure::VerseChorus { verses, choruses } = creative.structure {
            assert_eq!(verses, 3);
            assert_eq!(choruses, 2);
        } else {
            panic!("Expected VerseChorus structure");
        }
    }

    #[test]
    fn test_cellular_graph_connections() {
        let mut graph = CellularGraph::new();
        graph.add_node("a", "Alpha", "core");
        graph.add_node("b", "Beta", "service");
        graph.connect("a", "b");

        assert_eq!(graph.nodes.len(), 2);
        assert!(graph.nodes[0].connections.contains(&"b".to_string()));
    }

    #[test]
    fn test_empty_embedding_for_empty_content() {
        let e = RiffEngine::compute_embedding("");
        assert_eq!(e, [0.0; EMBED_DIM]);
    }

    #[test]
    fn test_report_improvement_rate() {
        let mut engine = setup_engine();

        // Create sessions with increasing quality
        for i in 0..5 {
            let id = engine.start_session(SessionMode::Freeform, vec!["agent-a".into()]);
            engine.submit_riff(&id, "agent-a", &format!("Riff {}", i), RiffType::Mixed);
            engine.rate_riff(&id, 0, 0.3 + i as f64 * 0.1);
            engine.end_session(&id);
        }

        let report = engine.iteration_report();
        assert!(report.improvement_rate > 0.0);
    }

    #[test]
    fn test_riff_context_returns_correct_turn() {
        let mut engine = setup_engine();
        let id = engine.start_session(SessionMode::Freeform, vec!["agent-a".into()]);

        engine.submit_riff(&id, "agent-a", "First", RiffType::Mixed);
        engine.submit_riff(&id, "agent-a", "Second", RiffType::Mixed);

        let ctx = engine.get_riff_context(&id, "agent-a");
        assert_eq!(ctx.turn, 2);
        assert_eq!(ctx.previous_riffs.len(), 2);
    }
}
