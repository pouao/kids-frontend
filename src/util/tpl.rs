use axum::response::Html;
use handlebars::{
    Handlebars, Context, Helper, Output, RenderContext, RenderError,
};

use super::common::{scripts_dir, tpls_dir, get_lang_msg};

pub struct Hbs<'hbs> {
    pub name: String,
    pub reg: Handlebars<'hbs>,
}

impl<'hbs> Hbs<'hbs> {
    pub async fn new(rel_path: &str) -> Hbs<'hbs> {
        let tpl_name = rel_path.replace("/", "_");
        let abs_path =
            format!("{}{}.html", &tpls_dir().await, &rel_path);

        // create the handlebars registry
        let mut hbs_reg = Handlebars::new();
        // register template from a file and assign a name to it
        hbs_reg
            .register_template_file(&tpl_name, &abs_path)
            .unwrap();

        Hbs {
            name: tpl_name,
            reg: hbs_reg,
        }
    }

    pub async fn render<T>(&self, data: &T) -> Html<String>
    where
        T: serde::Serialize,
    {
        Html(self.reg.render(&self.name, data).unwrap())
    }

    pub async fn reg_head(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_template_file(
                "head",
                format!("{}{}", tpls_dir().await, "common/head.html"),
            )
            .unwrap();

        self
    }

    pub async fn reg_header(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_template_file(
                "nav-global",
                format!(
                    "{}{}",
                    tpls_dir().await,
                    "common/nav-global.html"
                ),
            )
            .unwrap();
        self.reg
            .register_template_file(
                "sign",
                format!("{}{}", tpls_dir().await, "common/sign.html"),
            )
            .unwrap();
        self.reg
            .register_template_file(
                "sign-popover",
                format!(
                    "{}{}",
                    tpls_dir().await,
                    "common/sign-popover.html"
                ),
            )
            .unwrap();
        self.reg
            .register_template_file(
                "header",
                format!(
                    "{}{}",
                    tpls_dir().await,
                    "common/header.html"
                ),
            )
            .unwrap();

        self
    }

    pub async fn reg_container(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_template_file(
                "wish-random",
                format!(
                    "{}{}",
                    tpls_dir().await,
                    "common/wish-random.html"
                ),
            )
            .unwrap();

        self
    }

    pub async fn reg_pagination(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_template_file(
                "pagination",
                format!(
                    "{}{}",
                    tpls_dir().await,
                    "common/pagination.html"
                ),
            )
            .unwrap();

        self
    }

    pub async fn reg_footer(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_template_file(
                "footer",
                format!(
                    "{}{}",
                    tpls_dir().await,
                    "common/footer.html"
                ),
            )
            .unwrap();

        self
    }

    pub async fn reg_script_values(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_script_helper_file(
                "helper-values",
                format!(
                    "{}{}",
                    scripts_dir().await,
                    "values/helper-values.rhai"
                ),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "str-cmp",
                format!(
                    "{}{}",
                    scripts_dir().await,
                    "values/str-cmp.rhai"
                ),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "str-cut",
                format!(
                    "{}{}",
                    scripts_dir().await,
                    "values/str-cut.rhai"
                ),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "value-check",
                format!(
                    "{}{}",
                    scripts_dir().await,
                    "values/value-check.rhai"
                ),
            )
            .unwrap();

        self
    }

    pub async fn reg_script_ops(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_script_helper_file(
                "add-op",
                format!(
                    "{}{}",
                    scripts_dir().await,
                    "ops/add-op.rhai"
                ),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "level-op",
                format!(
                    "{}{}",
                    scripts_dir().await,
                    "ops/level-op.rhai"
                ),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "sci-format",
                format!(
                    "{}{}",
                    scripts_dir().await,
                    "ops/sci-format.rhai"
                ),
            )
            .unwrap();

        self
    }

    pub async fn reg_script_lang(&mut self) -> &mut Hbs<'hbs> {
        self.reg.register_helper("lang", Box::new(lang_helper));

        self
    }
}

fn lang_helper(
    helper: &Helper,
    _hbs: &Handlebars,
    c: &Context,
    rc: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let lang_id = if let Some(language_val) = c.data().get("language")
    {
        language_val.as_str().unwrap_or_default()
    } else {
        "en-us"
    };

    let root_tpl = rc.get_root_template_name().unwrap().as_str();

    let msg_id = if let Some(msg_json) = helper.param(0) {
        msg_json.value().as_str().unwrap_or_default()
    } else {
        "{{ lang }} must have at least one parameter"
    };
    let msg_args = if helper.params().len() > 1 {
        Some(helper.param(1).unwrap().value().as_object().unwrap())
    } else {
        None
    };

    let value = get_lang_msg(lang_id, root_tpl, msg_id, msg_args);
    out.write(&value)?;

    Ok(())
}
