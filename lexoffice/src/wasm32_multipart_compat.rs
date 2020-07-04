struct Form {}
struct Part {}
trait Multipart {
    fn multipart(self, mut multipart: Form);
}
