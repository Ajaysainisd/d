use std::collections::HashMap;

use crate::config::ProjectConfig;
use crate::platform::Platform;
use crate::types::{CommandDef, ResolutionSource, ResolvedCommand};

pub fn resolve_command(
    verb: &str,
    target: Option<&str>,
    variant: Option<&str>,
    platform: Option<&Box<dyn Platform>>,
    project_config: Option<&ProjectConfig>,
    cli_env: &HashMap<String, String>,
) -> Option<ResolvedCommand> {
    let target_opt = target.map(|t| t.to_string());
    let variant_opt = variant.map(|v| v.to_string());

    // Layer 2: d.yaml project config overrides
    if let Some(config) = project_config {
        let parsed = crate::config::parse_d_yaml_override(config, verb, &target_opt, &variant_opt);
        if let Some(cmd_def) = parsed {
            return Some(build_resolved(
                verb,
                &target_opt,
                &variant_opt,
                &cmd_def,
                ResolutionSource::ProjectConfig,
                cli_env,
            ));
        }
    }

    // Layer 4: Platform defaults
    if let Some(p) = platform {
        let commands = p.commands();
        let best = find_best_match(verb, target, variant, &commands);
        if let Some(cmd_def) = best {
            return Some(build_resolved(
                verb,
                &target_opt,
                &variant_opt,
                &cmd_def,
                ResolutionSource::PlatformDefault,
                cli_env,
            ));
        }
    }

    // Layer 5: Built-in universal defaults
    let universal = builtin_commands();
    let best = find_best_match(verb, target, variant, &universal);
    best.map(|cmd_def| {
        build_resolved(
            verb,
            &target_opt,
            &variant_opt,
            &cmd_def,
            ResolutionSource::BuiltInDefault,
            cli_env,
        )
    })
}

fn build_resolved(
    verb: &str,
    target: &Option<String>,
    variant: &Option<String>,
    cmd_def: &CommandDef,
    source: ResolutionSource,
    cli_env: &HashMap<String, String>,
) -> ResolvedCommand {
    let args = substitute_placeholders(&cmd_def.args_template, target, variant);
    let mut env_vars = cmd_def.env_vars.clone();
    env_vars.extend(cli_env.clone());

    ResolvedCommand {
        verb: verb.to_string(),
        target: target.clone(),
        variant: variant.clone(),
        executable: cmd_def.executable.clone(),
        args,
        env_vars,
        working_dir: None,
        source,
    }
}

fn find_best_match<'a>(
    verb: &str,
    target: Option<&str>,
    variant: Option<&str>,
    commands: &'a [CommandDef],
) -> Option<&'a CommandDef> {
    let verb_matches: Vec<&CommandDef> = commands
        .iter()
        .filter(|c| c.verb == verb)
        .collect();

    if verb_matches.is_empty() {
        return None;
    }

    // No target given — return the first verb match that has no targets
    // or requires no target
    if target.is_none() {
        let no_target = verb_matches.iter().find(|c| c.targets.is_empty());
        if let Some(found) = no_target {
            return Some(found);
        }
        return verb_matches.first().copied();
    }

    let t = target.unwrap();

    // Find exact target match
    let target_match = verb_matches.iter().find(|c| c.targets.contains(&t.to_string()));
    if let Some(found) = target_match {
        // If variant given, prefer one that supports it
        if let Some(v) = variant {
            let variant_match = verb_matches.iter().find(|c| {
                c.targets.contains(&t.to_string()) && c.variants.contains(&v.to_string())
            });
            if let Some(vm) = variant_match {
                return Some(vm);
            }
        }
        return Some(found);
    }

    // Fallback to first verb match
    verb_matches.first().copied()
}

fn substitute_placeholders(
    template: &[String],
    target: &Option<String>,
    variant: &Option<String>,
) -> Vec<String> {
    template
        .iter()
        .map(|arg| {
            let mut s = arg.clone();
            if let Some(ref t) = target {
                s = s.replace("{target}", t);
            }
            if let Some(ref v) = variant {
                s = s.replace("{variant}", v);
            }
            s
        })
        .collect()
}

fn builtin_commands() -> Vec<CommandDef> {
    vec![
        CommandDef::new("doctor", "d", "Run project health checks"),
        CommandDef::new("help", "d", "Show help for a command").with_args(&["help"]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_no_platform_no_config() {
        let result = resolve_command("run", Some("web"), None, None, None, &HashMap::new());
        assert!(result.is_none());
    }

    #[test]
    fn test_substitute_placeholders() {
        let template = vec![
            "run".to_string(),
            "-d".to_string(),
            "{target}".to_string(),
        ];
        let result = substitute_placeholders(&template, &Some("chrome".into()), &None);
        assert_eq!(result, vec!["run", "-d", "chrome"]);
    }

    #[test]
    fn test_builtin_doctor() {
        let cmds = builtin_commands();
        assert!(cmds.iter().any(|c| c.verb == "doctor"));
    }
}
