use axum::response::Html;
use handlebars::{
    Handlebars, Context, Helper, Output, RenderContext, RenderError,
};

use super::{
    constant::{TPLS_DIR, SCRIPTS_DIR},
    common::get_lang_msg,
};

pub struct Hbs<'hbs> {
    pub name: String,
    pub reg: Handlebars<'hbs>,
}

impl<'hbs> Hbs<'hbs> {
    pub async fn new(rel_path: &str) -> Hbs<'hbs> {
        let tpl_name = rel_path.replace("/", "_");
        let abs_path = format!("{}/{}.html", TPLS_DIR, &rel_path);

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
                format!("{}/{}", TPLS_DIR, "common/head.html"),
            )
            .unwrap();

        self
    }

    pub async fn reg_header(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_template_file(
                "nav-global",
                format!("{}/{}", TPLS_DIR, "common/nav-global.html"),
            )
            .unwrap();
        self.reg
            .register_template_file(
                "sign",
                format!("{}/{}", TPLS_DIR, "common/sign.html"),
            )
            .unwrap();
        self.reg
            .register_template_file(
                "sign-popover",
                format!(
                    "{}/{}",
                    TPLS_DIR, "common/sign-popover.html"
                ),
            )
            .unwrap();
        self.reg
            .register_template_file(
                "header",
                format!("{}/{}", TPLS_DIR, "common/header.html"),
            )
            .unwrap();

        self
    }

    pub async fn reg_container(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_template_file(
                "wish-random",
                format!("{}/{}", TPLS_DIR, "common/wish-random.html"),
            )
            .unwrap();

        self
    }

    pub async fn reg_pagination(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_template_file(
                "pagination",
                format!("{}/{}", TPLS_DIR, "common/pagination.html"),
            )
            .unwrap();

        self
    }

    pub async fn reg_footer(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_template_file(
                "footer",
                format!("{}/{}", TPLS_DIR, "common/footer.html"),
            )
            .unwrap();

        self
    }

    pub async fn reg_script_values(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_script_helper_file(
                "helper-values",
                format!(
                    "{}/{}",
                    SCRIPTS_DIR, "values/helper-values.rhai"
                ),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "str-cmp",
                format!("{}/{}", SCRIPTS_DIR, "values/str-cmp.rhai"),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "str-cut",
                format!("{}/{}", SCRIPTS_DIR, "values/str-cut.rhai"),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "value-check",
                format!(
                    "{}/{}",
                    SCRIPTS_DIR, "values/value-check.rhai"
                ),
            )
            .unwrap();

        self
    }

    pub async fn reg_script_ops(&mut self) -> &mut Hbs<'hbs> {
        self.reg
            .register_script_helper_file(
                "add-op",
                format!("{}/{}", SCRIPTS_DIR, "ops/add-op.rhai"),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "level-op",
                format!("{}/{}", SCRIPTS_DIR, "ops/level-op.rhai"),
            )
            .unwrap();
        self.reg
            .register_script_helper_file(
                "sci-format",
                format!("{}/{}", SCRIPTS_DIR, "ops/sci-format.rhai"),
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
