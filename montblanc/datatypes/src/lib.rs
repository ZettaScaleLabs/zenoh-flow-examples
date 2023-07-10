//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

use prost::Message;
use rand::distributions::{Alphanumeric, Distribution, Standard};
use rand::{random, Rng};
use std::io::Cursor;
use std::time::{SystemTime, UNIX_EPOCH};

pub static AMAZON_PORT: &str = "Amazon";
pub static DANUBE_PORT: &str = "Danube";
pub static GANGES_PORT: &str = "Ganges";
pub static NILE_PORT: &str = "Nile";
pub static TIGRIS_PORT: &str = "Tigris";
pub static PARANA_PORT: &str = "Parana";
pub static COLUMBIA_PORT: &str = "Columbia";
pub static COLORADO_PORT: &str = "Colorado";
pub static SALWEEN_PORT: &str = "Salween";
pub static GODAVARI_PORT: &str = "Godavari";
pub static CHENAB_PORT: &str = "Chenab";
pub static LOIRE_PORT: &str = "Loire";
pub static YAMUNA_PORT: &str = "Yamuna";
pub static BRAZOS_PORT: &str = "Brazos";
pub static TAGUS_PORT: &str = "Tagus";
pub static MISSOURI_PORT: &str = "Missouri";
pub static CONGO_PORT: &str = "Congo";
pub static MEKONG_PORT: &str = "Mekong";
pub static ARKANSAS_PORT: &str = "Arkansas";
pub static OHIO_PORT: &str = "Ohio";
pub static VOLGA_PORT: &str = "Volga";
pub static MURRAY_PORT: &str = "Murray";
pub static LENA_PORT: &str = "Lena";

pub mod data_types {
    include!(concat!(env!("OUT_DIR"), "/datatypes.data_types.rs"));
}

pub fn random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn random_bytes(length: usize) -> Vec<u8> {
    (0..length).map(|_| rand::random::<u8>()).collect()
}

pub fn empty_bytes() -> Vec<u8> {
    Vec::new()
}

pub fn random_floats(length: usize) -> Vec<f32> {
    (0..length).map(|_| rand::random::<f32>()).collect()
}

pub fn random_doubles(length: usize) -> Vec<f64> {
    (0..length).map(|_| rand::random::<f64>()).collect()
}

impl Distribution<data_types::Header> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> data_types::Header {
        let now = SystemTime::now();
        let now_as_duration = now
            .duration_since(UNIX_EPOCH)
            .expect("System time went backwards");
        data_types::Header {
            sec: now_as_duration.as_secs() as i32,
            nanosec: now_as_duration.subsec_nanos(),
            frame_id: random_string(16),
        }
    }
}

impl Distribution<data_types::Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> data_types::Point {
        data_types::Point {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }
}

pub fn serialize_point(point: &data_types::Point) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.resize(point.encoded_len(), 0);
    point.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_point(buf: &[u8]) -> Result<data_types::Point, prost::DecodeError> {
    data_types::Point::decode(&mut Cursor::new(buf))
}

impl Distribution<data_types::Quaternion> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> data_types::Quaternion {
        data_types::Quaternion {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
            w: rng.gen(),
        }
    }
}

pub fn serialize_quaternion(quat: &data_types::Quaternion) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.resize(quat.encoded_len(), 0);
    quat.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_quaternion(buf: &[u8]) -> Result<data_types::Quaternion, prost::DecodeError> {
    data_types::Quaternion::decode(&mut Cursor::new(buf))
}

impl Distribution<data_types::Vector3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> data_types::Vector3 {
        data_types::Vector3 {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }
}

pub fn serialize_vector3(vec3: &data_types::Vector3) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.resize(vec3.encoded_len(), 0);
    vec3.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_vector3(buf: &[u8]) -> Result<data_types::Vector3, prost::DecodeError> {
    data_types::Vector3::decode(&mut Cursor::new(buf))
}

impl Distribution<data_types::Vector3Stamped> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> data_types::Vector3Stamped {
        data_types::Vector3Stamped {
            header: rng.gen(),
            vector: rng.gen(),
        }
    }
}

pub fn serialize_vector3_stamped(vec3s: &data_types::Vector3Stamped) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.resize(vec3s.encoded_len(), 0);
    vec3s.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_vector3_stamped(
    buf: &[u8],
) -> Result<data_types::Vector3Stamped, prost::DecodeError> {
    data_types::Vector3Stamped::decode(&mut Cursor::new(buf))
}

impl Distribution<data_types::Pose> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> data_types::Pose {
        data_types::Pose {
            position: rng.gen(),
            orientation: rng.gen(),
        }
    }
}

pub fn serialize_pose(pose: &data_types::Pose) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.resize(pose.encoded_len(), 0);
    pose.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_pose(buf: &[u8]) -> Result<data_types::Pose, prost::DecodeError> {
    data_types::Pose::decode(&mut Cursor::new(buf))
}

impl Distribution<data_types::Twist> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> data_types::Twist {
        data_types::Twist {
            linear: rng.gen(),
            angular: rng.gen(),
        }
    }
}

pub fn serialize_twist(twist: &data_types::Twist) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.resize(twist.encoded_len(), 0);
    twist.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_twist(buf: &[u8]) -> Result<data_types::Twist, prost::DecodeError> {
    data_types::Twist::decode(&mut Cursor::new(buf))
}

impl Distribution<data_types::TwistWithCovariance> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> data_types::TwistWithCovariance {
        data_types::TwistWithCovariance {
            twist: rng.gen(),
            covariance: random_doubles(36),
        }
    }
}

pub fn serialize_twist_with_covariance(
    twist_with_cov: &data_types::TwistWithCovariance,
) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.resize(twist_with_cov.encoded_len(), 0);
    twist_with_cov.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_twist_with_covariance(
    buf: &[u8],
) -> Result<data_types::TwistWithCovariance, prost::DecodeError> {
    data_types::TwistWithCovariance::decode(&mut Cursor::new(buf))
}

impl Distribution<data_types::TwistWithCovarianceStamped> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> data_types::TwistWithCovarianceStamped {
        data_types::TwistWithCovarianceStamped {
            header: random(),
            twist: rng.gen(),
        }
    }
}

pub fn serialize_twist_with_covariance_stamped(
    twist_with_cov: &data_types::TwistWithCovarianceStamped,
) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.resize(twist_with_cov.encoded_len(), 0);
    twist_with_cov.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_twist_with_covariance_stamped(
    buf: &[u8],
) -> Result<data_types::TwistWithCovarianceStamped, prost::DecodeError> {
    data_types::TwistWithCovarianceStamped::decode(&mut Cursor::new(buf))
}

impl Distribution<data_types::Wrench> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> data_types::Wrench {
        data_types::Wrench {
            force: rng.gen(),
            torque: rng.gen(),
        }
    }
}

pub fn serialize_wrench(wrench: &data_types::Wrench) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.resize(wrench.encoded_len(), 0);
    wrench.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_wrench(buf: &[u8]) -> Result<data_types::Wrench, prost::DecodeError> {
    data_types::Wrench::decode(&mut Cursor::new(buf))
}

impl Distribution<data_types::WrenchStamped> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> data_types::WrenchStamped {
        data_types::WrenchStamped {
            header: rng.gen(),
            wrench: rng.gen(),
        }
    }
}

pub fn serialize_wrench_stamped(wrench_stamped: &data_types::WrenchStamped) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.resize(wrench_stamped.encoded_len(), 0);
    wrench_stamped.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_wrench_stamped(
    buf: &[u8],
) -> Result<data_types::WrenchStamped, prost::DecodeError> {
    data_types::WrenchStamped::decode(&mut Cursor::new(buf))
}

impl Distribution<data_types::Image> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> data_types::Image {
        data_types::Image {
            header: rng.gen(),
            height: rng.gen(),
            width: rng.gen(),
            encoding: random_string(32),
            is_bigendian: rng.gen(),
            step: rng.gen(),
            //data: random_bytes(1920 * 1080 * 3),
            data: empty_bytes(),
        }
    }
}

pub fn serialize_image(img: &data_types::Image) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.resize(img.encoded_len(), 0);
    img.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_image(buf: &[u8]) -> Result<data_types::Image, prost::DecodeError> {
    data_types::Image::decode(&mut Cursor::new(buf))
}

impl Distribution<data_types::point_cloud2::point_field::DataType> for Standard {
    fn sample<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
    ) -> data_types::point_cloud2::point_field::DataType {
        match rng.gen_range(0..=7) {
            0 => data_types::point_cloud2::point_field::DataType::Int8,
            1 => data_types::point_cloud2::point_field::DataType::Uint8,
            2 => data_types::point_cloud2::point_field::DataType::Int16,
            3 => data_types::point_cloud2::point_field::DataType::Uint16,
            4 => data_types::point_cloud2::point_field::DataType::Int32,
            5 => data_types::point_cloud2::point_field::DataType::Uint32,
            6 => data_types::point_cloud2::point_field::DataType::Float32,
            7 => data_types::point_cloud2::point_field::DataType::Float64,
            _ => data_types::point_cloud2::point_field::DataType::Int8,
        }
    }
}

impl Distribution<data_types::point_cloud2::PointField> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> data_types::point_cloud2::PointField {
        data_types::point_cloud2::PointField {
            name: random_string(32),
            offset: rng.gen(),
            datatype: rng.gen(),
            count: rng.gen(),
        }
    }
}

fn random_point_fields(length: usize) -> Vec<data_types::point_cloud2::PointField> {
    (0..length)
        .map(|_| rand::random::<data_types::point_cloud2::PointField>())
        .collect()
}

impl Distribution<data_types::PointCloud2> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> data_types::PointCloud2 {
        data_types::PointCloud2 {
            header: rng.gen(),
            height: rng.gen(),
            width: rng.gen(),
            fields: random_point_fields(3),
            is_bigendian: rng.gen(),
            point_step: rng.gen(),
            row_step: rng.gen(),
            //data: random_bytes(4 * 4 * 4 * 1280 * 960),
            data: empty_bytes(),
            is_dense: rng.gen(),
        }
    }
}

pub fn serialize_pointcloud2(pc: &data_types::PointCloud2) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.resize(pc.encoded_len(), 0);
    pc.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_pointcloud2(buf: &[u8]) -> Result<data_types::PointCloud2, prost::DecodeError> {
    data_types::PointCloud2::decode(&mut Cursor::new(buf))
}

impl Distribution<data_types::LaserScan> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> data_types::LaserScan {
        data_types::LaserScan {
            header: rng.gen(),
            angle_min: rng.gen(),
            angle_max: rng.gen(),
            angle_increment: rng.gen(),
            time_increment: rng.gen(),
            scan_time: rng.gen(),
            range_min: rng.gen(),
            range_max: rng.gen(),
            ranges: random_floats(1024),
            intensities: random_floats(1024),
        }
    }
}

pub fn serialize_laserscan(ls: &data_types::LaserScan) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.resize(ls.encoded_len(), 0);
    ls.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_laserscan(buf: &[u8]) -> Result<data_types::LaserScan, prost::DecodeError> {
    data_types::LaserScan::decode(&mut Cursor::new(buf))
}
