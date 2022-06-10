pub trait Pipe {}

pub mod stair {
    use bson::oid::ObjectId;
    use mongodb::bson::DateTime as MongoDateTime;
    use regex::Regex;
    use thiserror::Error;

    use super::Pipe;

    pub trait Responder {
        fn response(&self) -> &Body;
    }

    pub trait Receiver {
        fn pattern_comparison(&self, text: &str) -> bool;
    }

    pub trait Step: Pipe + Receiver + Responder {
        fn kind(&self) -> &StageKind;
    }

    pub struct Way(Vec<Box<dyn Step>>);

    impl Way {
        pub fn new() -> Way {
            Self(Vec::new())
        }

        // Получить стартовое сообщение, если такое имеется
        pub fn start(&self) -> Option<&Box<dyn Step>> {
            self.0
                .iter()
                .find(|p| *p.kind() == StageKind::Start)
                .and_then(|stage| Some(stage))
        }

        /// Добавить новый шаг в список
        pub fn add(&mut self, value: Box<dyn Step>) -> &Way {
            self.0.push(value);
            self
        }

        /// Добавить несколько новых шагов в список
        pub fn add_many(&mut self, vector: Vec<Box<dyn Step>>) -> &Way {
            vector.into_iter().for_each(|value| self.0.push(value));
            self
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn define_by_pattern(&self, text: &str) -> Option<&Box<dyn Step>> {
            self.0.iter().find(|p| p.pattern_comparison(text))
        }
    }

    #[derive(Error, Debug)]
    pub enum StageError {
        #[error("invalid kind (expected `{0}`, found `{1}`)")]
        InvalidKind(StageKind, StageKind),
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum StageKind {
        /// Сообщение которое автоматически отправляется
        /// когда пользователь начинает диалог с ботом
        Start,

        // TODO
        State,

        /// Базовый тип сообщения, которые не требует
        /// фиксирования состояния диалога
        Basic,

        /// Пустой тип
        /// Необходим для того, чтобы было возможно создавать
        /// шаги заглушки, а после наполнять их контентом
        Empty,
    }

    impl std::fmt::Display for StageKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct Stage {
        #[serde(rename = "_id")]
        id: ObjectId,

        kind: StageKind,

        #[serde(with = "serde_regex")]
        pattern: Vec<Regex>,
        name: String,
        description: Option<String>,

        body: Option<Body>,

        created_at: i64,
        updated_at: i64,
    }

    impl Stage {
        pub fn new(
            kind: StageKind,
            pattern: Vec<Regex>,
            name: String,
            description: Option<String>,
            body: Option<Body>,
        ) -> Result<Box<dyn Step>, StageError> {
            Self::input_validation(&kind, &body)?;

            let stage = Self {
                id: bson::oid::ObjectId::new(),
                kind,
                pattern,
                name,
                description,

                body,

                created_at: MongoDateTime::now().timestamp_millis(),
                updated_at: MongoDateTime::now().timestamp_millis(),
            };

            Ok(Box::new(stage))
        }

        fn input_validation(kind: &StageKind, body: &Option<Body>) -> Result<(), StageError> {
            match kind {
                StageKind::Start if body.is_none() => {
                    Err(StageError::InvalidKind(StageKind::Start, StageKind::Empty))
                }
                StageKind::Start => Ok(()),

                StageKind::State => Ok(()),
                StageKind::Basic => Ok(()),

                StageKind::Empty if body.is_none() => Ok(()),
                StageKind::Empty => {
                    Err(StageError::InvalidKind(StageKind::Basic, StageKind::Empty))
                }
            }
        }
    }

    impl Step for Stage {
        fn kind(&self) -> &StageKind {
            &self.kind
        }
    }

    impl Receiver for Stage {
        fn pattern_comparison(&self, text: &str) -> bool {
            for p in self.pattern.iter() {
                if p.is_match(text) {
                    return true;
                }
            }

            return false;
        }
    }

    impl Responder for Stage {
        fn response(&self) -> &Body {
            &self.body.as_ref().unwrap()
        }
    }

    impl Pipe for Stage {}

    /// Полезная нагрузка шага
    /// Может содержать текст ответа, медиафайлы документы и другое
    #[derive(Serialize, Deserialize)]
    pub struct Body {
        text: Option<String>,
    }

    impl Body {
        pub fn new(text: Option<String>) -> Body {
            Self { text }
        }

        pub fn get_text(&self) -> &str {
            self.text.as_ref().map_or("", |t| t)
        }
    }
}
