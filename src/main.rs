
use bevy::prelude::*; // Bevy의 기본 프리루드를 가져옵니다. 게임 루프 및 주요 구조체들을 사용하기 위함입니다.
use bevy_prototype_lyon::prelude::*; // bevy_prototype_lyon을 사용해 벡터 형태의 2D 도형을 그리기 위해 필요합니다.
use std::f32::consts::PI; // PI 상수를 사용하기 위해 가져옵니다. (헥사곤의 각도 계산 시 유용)

const HEX_RADIUS: f32 = 50.0; // 각 헥사곤 타일의 반지름을 정의합니다. 이 값은 헥사곤의 크기를 결정합니다.

#[derive(Component)]
struct HexTile; // 헥사곤 타일에 붙일 컴포넌트입니다. 이를 통해 헥사곤 타일들을 쉽게 관리할 수 있습니다.

fn main() {
    App::new() // 새로운 Bevy 애플리케이션을 생성합니다.
        .add_plugins(DefaultPlugins) // Bevy의 기본 플러그인을 추가합니다. (렌더링, 윈도우 관리 등)
        .add_plugins(ShapePlugin) // Lyon의 ShapePlugin을 추가하여 도형 그리기 기능을 활성화합니다.
        .add_systems(Startup, setup) // setup 함수를 애플리케이션이 시작할 때 실행하도록 등록합니다.
        .run(); // 애플리케이션을 실행합니다.
}

// 시작 시 호출될 setup 시스템
fn setup(
    mut commands: Commands, // 엔티티를 생성하고 관리하는 데 사용하는 명령어 모음입니다.
    mut materials: ResMut<Assets<ColorMaterial>>, // 색상 재질을 관리하는 리소스입니다. (여기서는 사용되지 않지만 기본적인 구조를 위한 준비)
    mut meshes: ResMut<Assets<Mesh>>, // 메쉬 데이터를 관리하는 리소스입니다. (이 역시 사용되지 않지만 필요 시 추가할 수 있습니다)
) {
    // 카메라 추가: 2D 카메라를 생성하여 화면에 도형들을 표시할 수 있게 합니다.
    commands.spawn(Camera2dBundle::default());

    // 헥사곤 타일맵을 생성합니다. q와 r은 헥사곤 타일의 축 좌표를 나타냅니다.
    // -5에서 5까지의 범위로 타일맵을 생성하여 총 121개의 타일을 배치합니다.
    for q in -5..=5 {
        for r in -5..=5 {
            let position = hex_to_pixel(q, r, HEX_RADIUS); // 각 타일의 픽셀 좌표를 계산합니다.
            spawn_hex_tile(&mut commands, position); // 계산된 좌표를 사용해 헥사곤 타일을 스폰(생성)합니다.
        }
    }
}

// 하나의 헥사곤 타일을 생성하는 함수
fn spawn_hex_tile(commands: &mut Commands, position: Vec2) {
    // 헥사곤 모양을 정의합니다. RegularPolygon은 다각형을 정의하는 구조체입니다.
    // sides: 6 -> 6개의 변을 가진 헥사곤을 생성합니다.
    // feature: Radius(HEX_RADIUS) -> 반지름이 HEX_RADIUS인 헥사곤을 생성합니다.
    let hex_shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(HEX_RADIUS),
        ..Default::default()
    };

    // 도형을 화면에 스폰(생성)합니다.
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&hex_shape), // GeometryBuilder를 통해 정의한 모양(hex_shape)을 2D 경로로 빌드합니다.
            spatial: SpatialBundle { // 공간적 위치(Transform)를 설정합니다.
                transform: Transform::from_xyz(position.x, position.y, 0.0), // 헥사곤 타일의 위치를 지정합니다.
                ..default()
            },
            ..default()
        },
        Fill::color(Color::srgb(0.2, 0.7, 0.3)), // 헥사곤 타일을 채울 색을 지정합니다. (RGB 값으로 녹색 계열)
        HexTile, // 헥사곤 타일 컴포넌트를 추가하여 쉽게 추적 및 관리할 수 있도록 합니다.
    ));
}

// 헥사곤 좌표계를 2D 화면상의 픽셀 좌표로 변환하는 함수
fn hex_to_pixel(q: i32, r: i32, radius: f32) -> Vec2 {
    // 헥사곤 타일의 픽셀 좌표 계산:
    // q와 r은 헥사곤 좌표계를 나타냅니다. 이 좌표계를 2D 화면의 좌표로 변환합니다.
    let x = radius * 3.0_f32.sqrt() * (q as f32 + 0.5 * (r as f32)); // X 좌표 계산
    let y = radius * 1.5 * (r as f32); // Y 좌표 계산
    Vec2::new(x, y) // Vec2 구조체로 X와 Y 좌표를 반환합니다.
}