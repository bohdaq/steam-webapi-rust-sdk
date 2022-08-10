use std::fs;
use std::fs::{File, read_to_string};
use std::path::Path;
use std::io::Write;
use serde_json::Value;
use serde::Deserialize;
use crate::util::get_json_filetype;

#[derive(Deserialize, Debug)]
pub struct SteamAppDetails {
    pub app_id: i64,
    pub name: String,
    pub app_type: String,
    pub supported_languages: String,
    pub support_info: SupportInfo,
    pub short_description: String,
    pub screenshots: Vec<Screenshot>,
    pub reviews: String,
    pub required_age: i64,
    pub release_date: ReleaseDate,
    pub recommendations: Recommendations,
    pub price_overview: PriceOverview,
    pub platforms: Platforms,
    pub pc_requirements: PcRequirements,
    pub mac_requirements: MacRequirements,
    pub linux_requirements: LinuxRequirements,
    pub package_groups: Vec<PackageGroup>,
    pub(crate) detailed_description: String,
    pub(crate) header_image: String,
    pub(crate) website: String,
}

#[derive(Deserialize, Debug)]
pub struct SupportInfo {
    pub url: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct Screenshot {
    pub path_thumbnail: String,
    pub path_full: String,
    pub id: i64,
}

#[derive(Deserialize, Debug)]
pub struct ReleaseDate {
    pub date: String,
    pub coming_soon: bool,
}

#[derive(Deserialize, Debug)]
pub struct Recommendations {
    pub total: i64,
}

#[derive(Deserialize, Debug)]
pub struct PriceOverview {
    pub recurring_sub_desc: String,
    pub recurring_sub: i64,
    pub initial_formatted: String,
    pub initial: i64,
    pub final_formatted: String,
    pub final_price: i64,
    pub discount_percent: i64,
    pub currency: String,
}

#[derive(Deserialize, Debug)]
pub struct Platforms {
    pub windows: bool,
    pub mac: bool,
    pub linux: bool,
}

#[derive(Deserialize, Debug)]
pub struct PcRequirements {
    pub recommended: String,
    pub minimum: String,
}

#[derive(Deserialize, Debug)]
pub struct MacRequirements {
    pub recommended: String,
    pub minimum: String,
}

#[derive(Deserialize, Debug)]
pub struct LinuxRequirements {
    pub recommended: String,
    pub minimum: String,
}

#[derive(Deserialize, Debug)]
pub struct PackageGroup {
    pub title: String,
    pub selection_text: String,
    pub save_text: String,
    pub name: String,
    pub is_recurring_subscription: String,
    pub display_type: String,
    pub description: String,
    pub subs: Vec<Sub>,
}

#[derive(Deserialize, Debug)]
pub struct Sub {
    pub price_in_cents_with_discount: i64,
    pub percent_savings_text: String,
    pub percent_savings: i64,
    pub packageid: i64,
    pub option_text: String,
    pub option_description: String,
    pub is_free_license: bool,
    pub can_get_free_license: String,
}

pub fn get(app_id: i64) -> Result<SteamAppDetails, String> {
    let api_response_boxed = make_api_call(app_id);
    if api_response_boxed.is_err() {
        return Err(api_response_boxed.err().unwrap().to_string());
    } else {
        parse_api_call_result(api_response_boxed.unwrap(), app_id)
    }
}

pub fn get_cached(app_id: i64) -> Result<SteamAppDetails, String> {
    let filepath = get_resource_filepath(app_id);

    let boxed_read = read_to_string(filepath);
    let is_readable = boxed_read.is_ok();
    if is_readable {
        let cached_api_response = boxed_read.unwrap();
        parse_api_call_result(cached_api_response, app_id)
    } else {
        get(app_id)
    }

}

pub fn make_api_call(app_id: i64) -> Result<String, String> {
    let url = get_api_url(app_id);

    let boxed_response = minreq::get(url).send();
    if boxed_response.is_err() {
        return Err("Operation timed out (API call)".to_string());
    }

    let raw_response : Vec<u8> = boxed_response.unwrap().into_bytes();

    let response_string_boxed = String::from_utf8(raw_response);
    if response_string_boxed.is_err() {
        let error_message = response_string_boxed.err().unwrap().to_string();
        if error_message == "invalid utf-8 sequence of 1 bytes from index 1" {
            return Err("no response from API".to_string());
        }
        return Err("invalid utf-8 sequence".to_string());
    }
    let response_string: String = response_string_boxed.unwrap();

    Ok(response_string)
}

pub fn get_api_url(app_id: i64) -> String {
    let api_url = format!("https://store.steampowered.com/api/appdetails?appids={}&lang=en", app_id);
    api_url
}

pub fn get_resource_filepath(app_id: i64) -> String {
    let cache_dir = get_cache_dir_path(app_id);
    let filepath = [
        cache_dir,
        app_id.to_string(),
        ".".to_string(),
        get_json_filetype(),
    ].join("");
    filepath
}


pub fn parse_api_call_result(response_string: String, app_id: i64) -> Result<SteamAppDetails, String> {
    let mut steam_app_details = SteamAppDetails {
        app_id: app_id,
        name: "".to_string(),
        app_type: "".to_string(),
        supported_languages: "".to_string(),
        support_info: SupportInfo {
            url: "".to_string(),
            email: "".to_string()
        },
        short_description: "".to_string(),
        screenshots: vec![],
        detailed_description: "".to_string(),
        reviews: "".to_string(),
        header_image: "".to_string(),
        website: "".to_string(),
        required_age: 0,
        release_date: ReleaseDate {
            date: "".to_string(),
            coming_soon: false
        },
        recommendations: Recommendations {
            total: 0
        },
        price_overview: PriceOverview {
            recurring_sub_desc: "".to_string(),
            recurring_sub: 0,
            initial_formatted: "".to_string(),
            initial: 0,
            final_formatted: "".to_string(),
            final_price: 0,
            discount_percent: 0,
            currency: "".to_string()
        },
        platforms: Platforms {
            windows: false,
            mac: false,
            linux: false
        },
        pc_requirements: PcRequirements {
            recommended: "".to_string(),
            minimum: "".to_string()
        },
        mac_requirements: MacRequirements {
            recommended: "".to_string(),
            minimum: "".to_string()
        },
        linux_requirements: LinuxRequirements {
            recommended: "".to_string(),
            minimum: "".to_string()
        },
        package_groups: vec![],
    };

    if response_string.len() > 0 {
        let boxed_initial_parse = serde_json::from_str(&response_string);
        if boxed_initial_parse.is_err() {
            return Err(boxed_initial_parse.err().unwrap().to_string());
        }
        let mut json: Value = boxed_initial_parse.unwrap();

        let mut app_details_wrapped = json[app_id.to_string()].take();

        let mut is_success = app_details_wrapped["success".to_string()].take();
        if is_success.take().as_bool().unwrap() == false {
            return Err("steampowered api returned failed response".to_string());
        }

        let mut app_details : Value = app_details_wrapped["data"].take();

        let boxed_website = app_details["website"].take();
        if boxed_website.as_str().is_some() {
            steam_app_details.website = boxed_website.as_str().unwrap().to_string();
        }

        let boxed_type = app_details["type"].take();
        if boxed_type.as_str().is_some() {
            steam_app_details.app_type = boxed_type.as_str().unwrap().to_string();
        }

        let boxed_supported_languages = app_details["supported_languages"].take();
        if boxed_supported_languages.as_str().is_some() {
            steam_app_details.supported_languages = boxed_supported_languages.as_str().unwrap().to_string();
        }

        let boxed_support_info = app_details["support_info"].take();
        if boxed_support_info.as_object().is_some() {
            let support_info = boxed_support_info.as_object().unwrap();

            let url =  support_info.get("url").unwrap().as_str().unwrap();
            let email =  support_info.get("email").unwrap().as_str().unwrap();

            let support_info = SupportInfo {
                url: url.to_string(),
                email: email.to_string(),
            };

            steam_app_details.support_info = support_info;
        }


        let boxed_price_overview = app_details["price_overview"].take();
        if boxed_price_overview.as_object().is_some() {
            let price_overview_map = boxed_price_overview.as_object().unwrap();


            let mut price_overview = PriceOverview {
                recurring_sub_desc: "".to_string(),
                recurring_sub: 0,
                initial_formatted: "".to_string(),
                initial: 0,
                final_formatted: "".to_string(),
                final_price: 0,
                discount_percent: 0,
                currency: "".to_string()
            };


            let boxed_recurring_sub_desc = price_overview_map.get("recurring_sub_desc");
            if boxed_recurring_sub_desc.is_some() {
                price_overview.recurring_sub_desc = boxed_recurring_sub_desc.unwrap().as_str().unwrap().to_string();
            }


            let boxed_initial_formatted = price_overview_map.get("initial_formatted");
            if boxed_initial_formatted.is_some() {
                price_overview.initial_formatted =  boxed_initial_formatted.unwrap().as_str().unwrap().to_string();
            }


            let boxed_final_formatted = price_overview_map.get("final_formatted");
            if boxed_initial_formatted.is_some() {
                price_overview.final_formatted =  boxed_final_formatted.unwrap().as_str().unwrap().to_string();
            }


            let boxed_recurring_sub = price_overview_map.get("recurring_sub");
            if boxed_recurring_sub.is_some() {
                price_overview.recurring_sub = boxed_recurring_sub.unwrap().as_i64().unwrap();
            }


            let boxed_initial = price_overview_map.get("initial");
            if boxed_initial.is_some() {
                price_overview.initial = boxed_initial.unwrap().as_i64().unwrap();
            }


            let boxed_final_price = price_overview_map.get("final");
            if boxed_final_price.is_some() {
                price_overview.final_price = boxed_final_price.unwrap().as_i64().unwrap();
            }

            let boxed_discount_percent = price_overview_map.get("discount_percent");
            if boxed_discount_percent.is_some() {
                price_overview.discount_percent = boxed_discount_percent.unwrap().as_i64().unwrap();
            }


            let boxed_currency = price_overview_map.get("currency");
            if boxed_currency.is_some() {
                price_overview.currency = boxed_currency.unwrap().as_str().unwrap().to_string();
            }

            steam_app_details.price_overview = price_overview;
        }

        let boxed_platforms = app_details["platforms"].take();
        if boxed_platforms.as_object().is_some() {
            let platforms_map = boxed_platforms.as_object().unwrap();

            let mut platforms = Platforms {
                windows: false,
                mac: false,
                linux: false
            };

            let boxed_windows = platforms_map.get("windows");
            if boxed_windows.is_some() {
                platforms.windows = boxed_windows.unwrap().as_bool().unwrap();
            }

            let boxed_mac = platforms_map.get("mac");
            if boxed_mac.is_some() {
                platforms.mac = boxed_mac.unwrap().as_bool().unwrap();
            }

            let boxed_linux = platforms_map.get("linux");
            if boxed_linux.is_some() {
                platforms.linux = boxed_linux.unwrap().as_bool().unwrap();
            }


            steam_app_details.platforms = platforms;
        }

        let boxed_pc_requirements = app_details["pc_requirements"].take();
        if boxed_pc_requirements.as_object().is_some() {
            let pc_requirements_map = boxed_pc_requirements.as_object().unwrap();

            let mut pc_requirements = PcRequirements {
                recommended: "".to_string(),
                minimum: "".to_string()
            };

            let boxed_minimum = pc_requirements_map.get("minimum");
            if boxed_minimum.is_some() {
                pc_requirements.minimum = boxed_minimum.unwrap().as_str().unwrap().to_string();
            }


            let boxed_recommended = pc_requirements_map.get("recommended");
            if boxed_recommended.is_some() {
                pc_requirements.recommended = boxed_recommended.unwrap().as_str().unwrap().to_string();
            }


            steam_app_details.pc_requirements = pc_requirements;
        }


        let boxed_mac_requirements = app_details["mac_requirements"].take();
        if boxed_mac_requirements.as_object().is_some() {
            let mac_requirements_map = boxed_mac_requirements.as_object().unwrap();

            let mut mac_requirements = MacRequirements {
                recommended: "".to_string(),
                minimum: "".to_string()
            };

            let boxed_minimum = mac_requirements_map.get("minimum");
            if boxed_minimum.is_some() {
                mac_requirements.minimum = boxed_minimum.unwrap().as_str().unwrap().to_string();
            }


            let boxed_recommended = mac_requirements_map.get("recommended");
            if boxed_recommended.is_some() {
                mac_requirements.recommended = boxed_recommended.unwrap().as_str().unwrap().to_string();
            }


            steam_app_details.mac_requirements = mac_requirements;
        }

        let boxed_linux_requirements = app_details["linux_requirements"].take();
        if boxed_linux_requirements.as_object().is_some() {
            let linux_requirements_map = boxed_linux_requirements.as_object().unwrap();

            let mut linux_requirements = LinuxRequirements {
                recommended: "".to_string(),
                minimum: "".to_string()
            };

            let boxed_minimum = linux_requirements_map.get("minimum");
            if boxed_minimum.is_some() {
                linux_requirements.minimum = boxed_minimum.unwrap().as_str().unwrap().to_string();
            }


            let boxed_recommended = linux_requirements_map.get("recommended");
            if boxed_recommended.is_some() {
                linux_requirements.recommended = boxed_recommended.unwrap().as_str().unwrap().to_string();
            }


            steam_app_details.linux_requirements = linux_requirements;
        }


        let boxed_recommendations = app_details["recommendations"].take();
        if boxed_recommendations.as_object().is_some() {
            let recommendations_map = boxed_recommendations.as_object().unwrap();

            let total =  recommendations_map.get("total").unwrap().as_i64().unwrap();

            let recommendations = Recommendations {
                total
            };

            steam_app_details.recommendations = recommendations;
        }

        let boxed_release_date = app_details["release_date"].take();
        if boxed_release_date.as_object().is_some() {
            let release_date_map = boxed_release_date.as_object().unwrap();

            let date =  release_date_map.get("date").unwrap().as_str().unwrap();
            let coming_soon =  release_date_map.get("coming_soon").unwrap().as_bool().unwrap();

            let release_date = ReleaseDate {
                date: date.to_string(),
                coming_soon,
            };

            steam_app_details.release_date = release_date;
        }

        let boxed_required_age = app_details["required_age"].take();
        if boxed_required_age.as_str().is_some() {
            steam_app_details.required_age = boxed_required_age.as_str().unwrap().parse().unwrap();
        }
        if boxed_required_age.as_i64().is_some() {
            steam_app_details.required_age = boxed_required_age.as_i64().unwrap();
        }

        let boxed_short_description = app_details["short_description"].take();
        if boxed_short_description.as_str().is_some() {
            steam_app_details.short_description = boxed_short_description.as_str().unwrap().to_string();
        }

        let boxed_screenshots = app_details["screenshots"].take();
        if boxed_screenshots.as_array().is_some() {
            let mut screenshoot_parsed_list : Vec<Screenshot> = vec![];

            let screenshots_list = boxed_screenshots.as_array().unwrap();
            for screenshot_val in screenshots_list {
                let mut screenshot = Screenshot {
                    path_thumbnail: "".to_string(),
                    path_full: "".to_string(),
                    id: 0
                };
                screenshot.path_thumbnail = screenshot_val["path_thumbnail"].as_str().unwrap().to_string();
                screenshot.path_full = screenshot_val["path_full"].as_str().unwrap().to_string();
                screenshot.id = screenshot_val["id"].as_i64().unwrap();

                screenshoot_parsed_list.push(screenshot);
            }
            steam_app_details.screenshots = screenshoot_parsed_list;
        }

        let boxed_reviews = app_details["reviews"].take();
        if boxed_reviews.as_str().is_some() {
            steam_app_details.reviews = boxed_reviews.as_str().unwrap().to_string();
        }

        let boxed_name = app_details["name"].take();
        if boxed_name.as_str().is_some() {
            steam_app_details.name = boxed_name.as_str().unwrap().to_string();
        }

        let boxed_header_image = app_details["header_image"].take();
        if boxed_header_image.as_str().is_some() {
            steam_app_details.header_image = boxed_header_image.as_str().unwrap().to_string();
        }

        let boxed_detailed_description = app_details["detailed_description"].take();
        if boxed_detailed_description.as_str().is_some() {
            steam_app_details.detailed_description = boxed_detailed_description.as_str().unwrap().to_string();
        }

        steam_app_details.package_groups = parse_package_groups(app_details);

    }

    let filepath = get_resource_filepath(app_id);

    let mut file: File;
    let directory_exists = Path::new(get_cache_dir_path(app_id).as_str()).is_dir();
    if !directory_exists {
        fs::create_dir_all(get_cache_dir_path(app_id)).unwrap();
        file = File::create(filepath).unwrap();
    } else {
        file = File::create(filepath).unwrap();
    }

    file.write_all(response_string.as_ref()).unwrap();

    Ok(steam_app_details)
}

pub fn parse_package_groups(mut app_details: Value) -> Vec<PackageGroup> {
    let mut package_group_list: Vec<PackageGroup> = vec![];

    let boxed_package_groups = app_details["package_groups"].take();
    if boxed_package_groups.as_array().is_some() {
        let package_groups = boxed_package_groups.as_array().unwrap();
        let mut package_group = PackageGroup {
            title: "".to_string(),
            selection_text: "".to_string(),
            save_text: "".to_string(),
            name: "".to_string(),
            is_recurring_subscription: "".to_string(),
            display_type: "".to_string(),
            description: "".to_string(),
            subs: vec![]
        };

        for package_group_map in package_groups {
            let boxed_title = package_group_map.get("title");
            if boxed_title.is_some() {
                package_group.title = boxed_title.unwrap().as_str().unwrap().to_string();
            }

            let boxed_selection_text = package_group_map.get("selection_text");
            if boxed_selection_text.is_some() {
                package_group.selection_text = boxed_selection_text.unwrap().as_str().unwrap().to_string();
            }

            let boxed_name = package_group_map.get("name");
            if boxed_name.is_some() {
                package_group.name = boxed_name.unwrap().as_str().unwrap().to_string();
            }

            let boxed_save_text = package_group_map.get("save_text");
            if boxed_save_text.is_some() {
                package_group.save_text = boxed_save_text.unwrap().as_str().unwrap().to_string();
            }

            let boxed_is_recurring_subscription = package_group_map.get("is_recurring_subscription");
            if boxed_is_recurring_subscription.is_some() {
                package_group.is_recurring_subscription = boxed_is_recurring_subscription.unwrap().as_str().unwrap().to_string();
            }

            let boxed_display_type = package_group_map.get("display_type");
            if boxed_display_type.is_some() {
                let display_type_as_str = boxed_display_type.unwrap().as_str();
                if display_type_as_str.is_some() {
                    package_group.display_type = display_type_as_str.unwrap().to_string();
                }

                let display_type_as_i64 = boxed_display_type.unwrap().as_i64();
                if display_type_as_i64.is_some() {
                    package_group.display_type = display_type_as_i64.unwrap().to_string();
                }
            }

            let boxed_description = package_group_map.get("description");
            if boxed_description.is_some() {
                package_group.description = boxed_description.unwrap().as_str().unwrap().to_string();
            }

            package_group.subs = parse_package_groups_subs(&package_group_map);
        }

        package_group_list.push(package_group);
    }

    package_group_list
}

pub fn parse_package_groups_subs(package_group: &Value) -> Vec<Sub> {
    let mut sub_list: Vec<Sub> = vec![];

    let sub_as_array = package_group.get("subs").take();
    if sub_as_array.is_some() {

        let sub_list_value = sub_as_array.unwrap().as_array().unwrap();

        for sub_item in sub_list_value {

            let mut sub = Sub {
                price_in_cents_with_discount: 0,
                percent_savings_text: "".to_string(),
                percent_savings: 0,
                packageid: 0,
                option_text: "".to_string(),
                option_description: "".to_string(),
                is_free_license: false,
                can_get_free_license: "".to_string()
            };

            let boxed_price_in_cents_with_discount = sub_item.get("price_in_cents_with_discount");
            if boxed_price_in_cents_with_discount.is_some() {
                sub.price_in_cents_with_discount = boxed_price_in_cents_with_discount.unwrap().as_i64().unwrap();
            }

            let boxed_packageid = sub_item.get("packageid");
            if boxed_packageid.is_some() {
                sub.packageid = boxed_packageid.unwrap().as_i64().unwrap();
            }
            sub_list.push(sub);
        }
    }

    sub_list
}

pub fn get_cache_dir_path(app_id: i64) -> String {
    let  interface = "steampowered";
    let  method = "appdetails";
    let number_of_entries_per_bucket = 10000;
    let bucket = app_id / number_of_entries_per_bucket;

    [
        "steam-webapi-cache".to_string(),
        "/".to_string(),
        interface.to_string(),
        "/".to_string(),
        method.to_string(),
        "/".to_string(),
        bucket.to_string(),
        "/".to_string(),
        app_id.to_string(),
        "/".to_string()
    ].join("")
}